use {
  crate::{apps, errors::ImageOptimError},
  globset::{GlobBuilder, GlobMatcher},
  ignore::WalkBuilder,
  std::{
    path::{Path, PathBuf},
    sync::Arc,
  },
};

/// The compiled forms of every ! exclusion. Shared with each walk's filter
/// closure via Arc, which requires 'static ownership
struct Excludes {
  /// Match excluded files
  filters: Vec<GlobMatcher>,
  /// Match directories whose whole subtree is excluded, see
  /// [prunable_dir_pattern]
  prunes: Vec<GlobMatcher>,
}

impl Excludes {
  fn new(exclude_patterns: &[&String], home_dir: Option<&Path>) -> Result<Self, ImageOptimError> {
    let prepared: Vec<String> = exclude_patterns
      .iter()
      .map(|pattern| prepare_pattern(&pattern[1..], home_dir))
      .collect();
    Ok(Self {
      filters: compile_all(&prepared)?,
      prunes: compile_all(
        &prepared
          .iter()
          .map(|pattern| prunable_dir_pattern(pattern))
          .collect::<Vec<String>>(),
      )?,
    })
  }

  fn is_excluded(&self, path: &Path) -> bool {
    self.filters.iter().any(|exclude| exclude.is_match(path))
  }

  fn is_prunable(&self, dir: &Path) -> bool {
    self.prunes.iter().chain(&self.filters).any(|exclude| exclude.is_match(dir))
  }
}

/// Expand glob patterns into the sorted, deduplicated paths of every supported
/// image they match. Patterns starting with ! exclude files, as globby's did.
/// Defaults to every image in the current directory when only exclusions or no
/// patterns at all are given.
///
/// Each pattern's literal prefix is walked once, hidden files are skipped (as
/// globby's dot:false did), and subtrees matching an exclusion are pruned
/// without being walked
pub fn find_images(patterns: &[String], home_dir: Option<&Path>) -> Result<Vec<PathBuf>, ImageOptimError> {
  patterns.iter().try_for_each(|pattern| reject_unsupported_syntax(pattern))?;
  let (exclude_patterns, include_patterns): (Vec<&String>, Vec<&String>) = patterns.iter().partition(|pattern| pattern.starts_with('!'));
  let excludes = Arc::new(Excludes::new(&exclude_patterns, home_dir)?);
  let include_patterns: Vec<String> = if include_patterns.is_empty() {
    vec!["**/*".to_string()]
  } else {
    include_patterns.iter().map(|pattern| prepare_pattern(pattern, home_dir)).collect()
  };
  let mut files: Vec<PathBuf> = include_patterns
    .iter()
    .map(|pattern| matches_for(pattern, &excludes))
    .collect::<Result<Vec<Vec<PathBuf>>, ImageOptimError>>()?
    .into_iter()
    .flatten()
    .collect();
  files.sort();
  files.dedup();
  Ok(files)
}

/// Every existing, supported, non-excluded image one prepared pattern matches
fn matches_for(pattern: &str, excludes: &Arc<Excludes>) -> Result<Vec<PathBuf>, ImageOptimError> {
  if is_literal(pattern) {
    Ok(literal_match(pattern, excludes).into_iter().collect())
  } else {
    walk_matches(pattern, excludes)
  }
}

/// An explicit file path: no walk needed, and unlike the walk it may name a
/// hidden file
fn literal_match(pattern: &str, excludes: &Excludes) -> Option<PathBuf> {
  let path = normalize(Path::new(pattern));
  (path.is_file() && is_wanted(&path, excludes)).then_some(path)
}

/// Walk the pattern's literal prefix, pruning excluded subtrees, collecting
/// the files the pattern matches
fn walk_matches(pattern: &str, excludes: &Arc<Excludes>) -> Result<Vec<PathBuf>, ImageOptimError> {
  let matcher = compile(pattern)?;
  let root = walk_root(pattern);
  if !root.exists() {
    return Ok(vec![]);
  }
  let excludes_for_filter = Arc::clone(excludes);
  WalkBuilder::new(&root)
    .standard_filters(false)
    // A pattern which explicitly names a dot component opts that walk into
    // hidden entries, as micromatch's dot:false did for literal dots
    .hidden(!names_hidden_components(pattern))
    .follow_links(true)
    .filter_entry(move |entry| {
      !entry.file_type().is_some_and(|file_type| file_type.is_dir()) || !excludes_for_filter.is_prunable(&normalize(entry.path()))
    })
    .build()
    .filter_map(|entry| match entry.map_err(unreadable) {
      Err(err) => Some(Err(err)),
      Ok(entry) if !entry.file_type().is_some_and(|file_type| file_type.is_file()) => None,
      Ok(entry) => {
        let path = normalize(entry.path());
        (matcher.is_match(&path) && is_wanted(&path, excludes)).then_some(Ok(path))
      }
    })
    .collect()
}

fn is_wanted(path: &Path, excludes: &Excludes) -> bool {
  apps::is_supported_extension(&apps::SUPPORTED_EXTENSIONS, path) && !excludes.is_excluded(path)
}

/// Strip ./ components so the same file matched via different spellings
/// deduplicates, and so patterns match walker output. components() drops
/// interior ./ but keeps a leading one, which the walker adds when its root
/// is the current directory
fn normalize(path: &Path) -> PathBuf {
  path
    .components()
    .filter(|component| !matches!(component, std::path::Component::CurDir))
    .collect()
}

fn compile(pattern: &str) -> Result<GlobMatcher, ImageOptimError> {
  GlobBuilder::new(pattern)
    .literal_separator(true)
    .backslash_escape(true)
    .build()
    .map(|glob| glob.compile_matcher())
    .map_err(|err| ImageOptimError::InvalidPattern {
      pattern: pattern.to_string(),
      reason: err.kind().to_string(),
    })
}

fn compile_all(patterns: &[String]) -> Result<Vec<GlobMatcher>, ImageOptimError> {
  patterns.iter().map(|pattern| compile(pattern)).collect()
}

/// The directory form of an exclusion, so that its whole subtree can be
/// skipped during the walk: originals/** prunes at the originals directory.
/// Exclusions in file form (*.jpg) prune nothing and only filter matches
fn prunable_dir_pattern(pattern: &str) -> String {
  pattern.trim_end_matches("/**/*").trim_end_matches("/**").to_string()
}

/// micromatch syntax which globset does not implement: fail loudly instead of
/// silently matching nothing
fn reject_unsupported_syntax(pattern: &str) -> Result<(), ImageOptimError> {
  for extglob in ["@(", "!(", "+(", "?(", "*("] {
    if pattern.contains(extglob) {
      return Err(ImageOptimError::UnsupportedPattern {
        pattern: pattern.to_string(),
        reason: format!("Extended glob syntax '{extglob}...)' is not supported, use {{a,b}} alternation instead"),
      });
    }
  }
  if has_brace_range(pattern) {
    return Err(ImageOptimError::UnsupportedPattern {
      pattern: pattern.to_string(),
      reason: "Brace ranges like {1..3} are not supported, list the alternates instead: {1,2,3}".to_string(),
    });
  }
  Ok(())
}

/// True for {A..B} ranges, false for plain {a,b} alternation
fn has_brace_range(pattern: &str) -> bool {
  pattern
    .split_once('{')
    .and_then(|(_, after_open)| after_open.split_once('}'))
    .is_some_and(|(body, rest)| is_brace_range_body(body) || has_brace_range(rest))
}

/// The A..B between braces: both endpoints present, and no comma making it an
/// alternation
fn is_brace_range_body(body: &str) -> bool {
  !body.contains(',') && body.find("..").is_some_and(|dots| dots > 0 && dots + 2 < body.len())
}

/// True when any path component of the pattern literally starts with a dot,
/// such as */.thumbs/*.png
fn names_hidden_components(pattern: &str) -> bool {
  pattern
    .split('/')
    .any(|component| component.starts_with('.') && component != "." && component != "..")
}

/// The characters which make a pattern a glob rather than a plain file path
const GLOB_METACHARACTERS: &[char] = &['*', '?', '[', '{', '\\'];

/// A pattern with no glob metacharacters is a plain file path
fn is_literal(pattern: &str) -> bool {
  !pattern.contains(GLOB_METACHARACTERS)
}

/// The pattern's leading metacharacter-free directories: the smallest tree the
/// walk must cover
fn walk_root(pattern: &str) -> PathBuf {
  let root: PathBuf = Path::new(pattern)
    .components()
    .take_while(|component| !component.as_os_str().to_string_lossy().contains(GLOB_METACHARACTERS))
    .collect();
  if root.as_os_str().is_empty() { PathBuf::from(".") } else { root }
}

fn unreadable(err: ignore::Error) -> ImageOptimError {
  match err {
    ignore::Error::WithDepth { err, .. } => unreadable(*err),
    ignore::Error::WithPath { path, err } => ImageOptimError::UnreadablePath {
      path: path.display().to_string(),
      reason: err.to_string(),
    },
    other => ImageOptimError::UnreadablePath {
      path: "a matched path".to_string(),
      reason: other.to_string(),
    },
  }
}

fn prepare_pattern(pattern: &str, home_dir: Option<&Path>) -> String {
  let pattern = expand_directory(&expand_tilde(pattern, home_dir));
  // Match the normalized paths the walk produces: ./photos/**/* and
  // photos/**/* are the same pattern
  std::iter::successors(Some(pattern.as_str()), |pattern| pattern.strip_prefix("./"))
    .last()
    .unwrap_or(&pattern)
    .to_string()
}

/// Replace a leading ~ with the user's home directory
fn expand_tilde(pattern: &str, home_dir: Option<&Path>) -> String {
  match (pattern, home_dir) {
    ("~", Some(home_dir)) => home_dir.display().to_string(),
    (pattern, Some(home_dir)) => pattern
      .strip_prefix("~/")
      .map(|remainder| format!("{}/{remainder}", home_dir.display()))
      .unwrap_or_else(|| pattern.to_string()),
    (pattern, None) => pattern.to_string(),
  }
}

/// When a pattern is the path of a directory, match every file inside it. The
/// directory name is a literal, so glob metacharacters in it are escaped
fn expand_directory(pattern: &str) -> String {
  if Path::new(pattern).is_dir() {
    format!("{}/**/*", escape(pattern.trim_end_matches('/')))
  } else {
    pattern.to_string()
  }
}

/// Backslash-escape glob metacharacters so a literal path only matches itself
fn escape(literal: &str) -> String {
  literal
    .chars()
    .flat_map(|character| {
      let needs_escape = matches!(character, '*' | '?' | '[' | ']' | '{' | '}' | '\\');
      needs_escape.then_some('\\').into_iter().chain(std::iter::once(character))
    })
    .collect()
}

#[cfg(test)]
#[path = "discovery_test.rs"]
mod discovery_test;
