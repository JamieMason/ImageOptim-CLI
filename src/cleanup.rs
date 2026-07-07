use std::{
  path::{Path, PathBuf},
  sync::{Mutex, PoisonError},
};

/// Symlink directories which exist right now and must not be left behind if
/// the process is interrupted
static SYMLINK_DIRS: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());

pub fn register(dir: &Path) {
  lock().push(dir.to_path_buf());
}

pub fn deregister(dir: &Path) {
  lock().retain(|registered| registered != dir);
}

/// Called from the SIGINT handler: remove every registered directory
pub fn remove_registered() {
  for dir in lock().drain(..) {
    let _ = std::fs::remove_dir_all(&dir);
  }
}

fn lock() -> std::sync::MutexGuard<'static, Vec<PathBuf>> {
  SYMLINK_DIRS.lock().unwrap_or_else(PoisonError::into_inner)
}

#[cfg(test)]
#[path = "cleanup_test.rs"]
mod cleanup_test;
