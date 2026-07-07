use std::{
  io,
  path::{Path, PathBuf},
};

/// Side-effecting file system operations, injected so that tests can fake them
pub trait FileSystem: Sync {
  fn file_size(&self, path: &Path) -> io::Result<u64>;
  /// Fill dir with symlinks to files, mirroring their directory structure
  fn create_symlink_dir(&self, dir: &Path, files: &[PathBuf]) -> io::Result<()>;
  fn remove_dir(&self, dir: &Path) -> io::Result<()>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
  fn file_size(&self, path: &Path) -> io::Result<u64> {
    std::fs::metadata(path).map(|metadata| metadata.len())
  }

  fn create_symlink_dir(&self, dir: &Path, files: &[PathBuf]) -> io::Result<()> {
    for file in files {
      let target = match file.canonicalize() {
        Ok(target) => target,
        // The file vanished after it was measured: the size pass will warn
        Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
        Err(err) => return Err(err),
      };
      // Lay links out along the canonicalised path: free of .. components, so
      // nothing can escape dir, and identical for every spelling of the same
      // file
      let link = dir.join(as_relative(&target));
      if link.symlink_metadata().is_ok() {
        continue;
      }
      if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent)?;
      }
      std::os::unix::fs::symlink(target, link)?;
    }
    Ok(())
  }

  fn remove_dir(&self, dir: &Path) -> io::Result<()> {
    if dir.exists() {
      std::fs::remove_dir_all(dir)?;
    }
    Ok(())
  }
}

/// Mirror the TypeScript CLI's tmpdir layout, where each file sat at
/// join(tmpDir, filePath): absolute paths lose their leading / so that they
/// nest inside the symlink directory
fn as_relative(file: &Path) -> PathBuf {
  file
    .components()
    .filter(|component| !matches!(component, std::path::Component::RootDir | std::path::Component::Prefix(_)))
    .collect()
}

#[cfg(test)]
#[path = "fs_test.rs"]
mod fs_test;
