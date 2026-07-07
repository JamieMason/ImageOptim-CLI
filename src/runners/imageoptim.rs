use {
  crate::{
    apps,
    errors::ImageOptimError,
    logger,
    runners::{BinaryExists, ExecOutcome, exec},
  },
  std::{ffi::OsStr, io, path::PathBuf},
};

/// Run the ImageOptim binary over the given files
pub type RunImageOptim = Box<dyn Fn(&[PathBuf]) -> io::Result<ExecOutcome> + Sync>;

/// Runs ImageOptim.app's bundled binary over image files in place
pub struct ImageOptim {
  pub binary_exists: BinaryExists,
  pub run_imageoptim: RunImageOptim,
}

impl ImageOptim {
  pub fn live() -> Self {
    Self {
      binary_exists: Box::new(|| apps::imageoptim_path().exists()),
      run_imageoptim: Box::new(|files| {
        let program = apps::imageoptim_path();
        exec(
          program.as_os_str(),
          &files.iter().map(|file| file.as_os_str()).collect::<Vec<&OsStr>>(),
        )
      }),
    }
  }

  pub fn run(&self, files: &[PathBuf]) -> Result<(), ImageOptimError> {
    if files.is_empty() {
      return Ok(());
    }
    if !(self.binary_exists)() {
      return Err(ImageOptimError::ImageOptimNotInstalled);
    }
    // The ImageOptim binary has no -- separator, so stop paths which start
    // with a dash being read as flags
    let files: Vec<PathBuf> = files
      .iter()
      .map(|file| {
        if file.to_string_lossy().starts_with('-') {
          std::path::Path::new(".").join(file)
        } else {
          file.clone()
        }
      })
      .collect();
    let outcome = (self.run_imageoptim)(&files)?;
    match outcome.status {
      Some(0) => {
        logger::debug("ImageOptim has finished");
        Ok(())
      }
      status => Err(ImageOptimError::app_failed("ImageOptim", status.unwrap_or(-1), &outcome.stderr)),
    }
  }
}

#[cfg(test)]
#[path = "imageoptim_test.rs"]
mod imageoptim_test;
