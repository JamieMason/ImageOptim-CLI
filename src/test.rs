use {
  crate::{fs::FileSystem, runners::ExecOutcome},
  std::{
    io,
    path::{Path, PathBuf},
    sync::Mutex,
  },
};

pub fn ok_outcome() -> ExecOutcome {
  ExecOutcome {
    status: Some(0),
    stdout: String::new(),
    stderr: String::new(),
  }
}

pub fn outcome_with_status(status: i32) -> ExecOutcome {
  ExecOutcome {
    status: Some(status),
    stdout: String::new(),
    stderr: String::new(),
  }
}

/// Records every call without touching the real file system. Every file is 100
/// bytes unless listed as missing or vanishing
#[derive(Default)]
pub struct TestFileSystem {
  pub created_symlink_dirs: Mutex<Vec<(PathBuf, Vec<PathBuf>)>>,
  pub removed_dirs: Mutex<Vec<PathBuf>>,
  pub fail_create_symlink_dir: bool,
  /// Paths which never exist
  pub missing_paths: Vec<PathBuf>,
  /// Paths which exist for their first size reading, then vanish
  pub vanishing_paths: Vec<PathBuf>,
  pub size_reads: Mutex<Vec<PathBuf>>,
}

impl FileSystem for TestFileSystem {
  fn file_size(&self, path: &Path) -> io::Result<u64> {
    let mut reads = self.size_reads.lock().unwrap();
    let already_read = reads.iter().any(|read| read == path);
    reads.push(path.to_path_buf());
    if self.missing_paths.iter().any(|missing| missing == path)
      || (already_read && self.vanishing_paths.iter().any(|vanishing| vanishing == path))
    {
      return Err(io::Error::new(io::ErrorKind::NotFound, "vanished"));
    }
    Ok(100)
  }

  fn create_symlink_dir(&self, dir: &Path, files: &[PathBuf]) -> io::Result<()> {
    self.created_symlink_dirs.lock().unwrap().push((dir.to_path_buf(), files.to_vec()));
    if self.fail_create_symlink_dir {
      return Err(io::Error::other("disk full"));
    }
    Ok(())
  }

  fn remove_dir(&self, dir: &Path) -> io::Result<()> {
    self.removed_dirs.lock().unwrap().push(dir.to_path_buf());
    Ok(())
  }
}
