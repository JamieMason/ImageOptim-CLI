use {
  super::*,
  std::{fs, os::unix::fs::PermissionsExt},
  tempfile::TempDir,
};

#[test]
fn expands_tilde_alone_to_the_home_directory() {
  assert_eq!(expand_tilde("~", Some(Path::new("/Users/jamie"))), "/Users/jamie");
}

#[test]
fn expands_leading_tilde_slash_to_the_home_directory() {
  assert_eq!(
    expand_tilde("~/Desktop/*.png", Some(Path::new("/Users/jamie"))),
    "/Users/jamie/Desktop/*.png"
  );
}

#[test]
fn leaves_tilde_unchanged_when_home_directory_is_unknown() {
  assert_eq!(expand_tilde("~/Desktop", None), "~/Desktop");
}

#[test]
fn leaves_patterns_without_tilde_unchanged() {
  assert_eq!(expand_tilde("**/*.png", Some(Path::new("/Users/jamie"))), "**/*.png");
}

#[test]
fn does_not_expand_tilde_mid_pattern() {
  assert_eq!(expand_tilde("images/~backup", Some(Path::new("/Users/jamie"))), "images/~backup");
}

#[test]
fn expands_a_directory_path_to_a_recursive_pattern() {
  let dir = TempDir::new().unwrap();
  let path = dir.path().display().to_string();
  assert_eq!(expand_directory(&path), format!("{path}/**/*"));
  assert_eq!(expand_directory(&format!("{path}/")), format!("{path}/**/*"));
}

#[test]
fn leaves_non_directory_patterns_unchanged() {
  assert_eq!(expand_directory("**/*.png"), "**/*.png");
}

#[test]
fn finds_supported_images_and_ignores_everything_else() {
  let dir = TempDir::new().unwrap();
  fs::write(dir.path().join("pixel.png"), "png").unwrap();
  fs::write(dir.path().join("notes.txt"), "txt").unwrap();
  let pattern = format!("{}/**/*", dir.path().display());
  let files = find_images(&[pattern], None).unwrap();
  assert_eq!(files, vec![dir.path().join("pixel.png")]);
}

#[test]
fn returns_an_error_for_malformed_patterns() {
  assert!(matches!(
    find_images(&["photos/[".to_string()], None),
    Err(ImageOptimError::InvalidPattern { .. })
  ));
}

#[test]
fn escapes_glob_metacharacters_in_literal_paths() {
  assert_eq!(escape("Photos [2024]/a*b"), "Photos \\[2024\\]/a\\*b");
}

#[test]
fn walks_from_the_deepest_literal_directory_of_a_pattern() {
  assert_eq!(walk_root("**/*"), PathBuf::from("."));
  assert_eq!(walk_root("photos/**/*.png"), PathBuf::from("photos"));
  assert_eq!(walk_root("/Users/jamie/Desktop/**/*"), PathBuf::from("/Users/jamie/Desktop"));
  assert_eq!(walk_root("photos/*.png"), PathBuf::from("photos"));
}

#[test]
fn treats_patterns_without_metacharacters_as_file_paths() {
  assert!(is_literal("photos/hero.png"));
  assert!(is_literal(".hidden.png"));
  assert!(!is_literal("**/*"));
  assert!(!is_literal("photos/[2024]/*.png"));
}

#[test]
fn derives_prunable_directories_from_exclusions() {
  assert_eq!(prunable_dir_pattern("originals/**"), "originals");
  assert_eq!(prunable_dir_pattern("originals/**/*"), "originals");
  assert_eq!(prunable_dir_pattern("**/*.jpg"), "**/*.jpg");
}

#[test]
fn surfaces_unreadable_directories_instead_of_skipping_them() {
  let dir = TempDir::new().unwrap();
  let private = dir.path().join("private");
  fs::create_dir_all(&private).unwrap();
  fs::write(private.join("hidden.png"), "png").unwrap();
  let mut lock = fs::metadata(&private).unwrap().permissions();
  lock.set_mode(0o000);
  fs::set_permissions(&private, lock).unwrap();
  let pattern = format!("{}/**/*", dir.path().display());
  let result = find_images(&[pattern], None);
  let mut unlock = fs::metadata(&private).unwrap().permissions();
  unlock.set_mode(0o755);
  fs::set_permissions(&private, unlock).unwrap();
  assert!(matches!(result, Err(ImageOptimError::UnreadablePath { .. })), "{result:?}");
}

#[test]
fn detects_brace_ranges_but_not_alternation() {
  assert!(has_brace_range("img{1..3}.png"));
  assert!(has_brace_range("a{x..y}b"));
  assert!(!has_brace_range("brace-{a,b}.png"));
  assert!(!has_brace_range("no braces at all.png"));
  assert!(!has_brace_range("dots{..}but..outside{a,b}"));
}

#[test]
fn rejects_extglob_in_exclusion_patterns_too() {
  assert!(matches!(
    find_images(&["!*.@(png|jpg)".to_string()], None),
    Err(ImageOptimError::UnsupportedPattern { .. })
  ));
}

#[test]
fn recognises_patterns_which_name_hidden_components() {
  assert!(names_hidden_components("*/.thumbs/*.png"));
  assert!(names_hidden_components(".hidden*/x.png"));
  assert!(!names_hidden_components("**/*"));
  assert!(!names_hidden_components("./photos/*.png"));
  assert!(!names_hidden_components("../photos/*.png"));
}
