use {
  crate::{
    apps::{App, JPEGMINI_VARIANTS},
    cleanup,
    errors::ImageOptimError,
    fs::FileSystem,
    logger,
    reporter::Reporter,
    runners::osascript::{IS_INSTALLED, QUIT_APP, RUN_JPEGMINI, SUPPORTS_ASSISTIVE_DEVICES, run_script},
  },
  std::{
    io,
    path::{Path, PathBuf},
  },
  uuid::Uuid,
};

/// Per-user temp location, as the TypeScript CLI used: a fixed shared path
/// like /var/tmp would be owned by whichever user ran first and break for
/// everyone else on the machine
fn symlink_dir_base() -> PathBuf {
  std::env::temp_dir().join("imageoptim-cli")
}

/// Automates whichever JPEGmini.app variant is installed over a temporary
/// directory of symlinks to the JPEG files, so that JPEGmini's edits land in
/// the original files without copying them anywhere
pub struct JpegMini {
  pub is_installed: IsInstalled,
  pub supports_assistive_devices: SupportsAssistiveDevices,
  pub automate_folder: AutomateFolder,
  pub quit: QuitApp,
}

/// Ask Finder whether an app variant is installed, by bundle id
pub type IsInstalled = Box<dyn Fn(&App) -> io::Result<bool> + Sync>;

/// Ask System Events whether GUI automation is permitted
pub type SupportsAssistiveDevices = Box<dyn Fn() -> io::Result<bool> + Sync>;

/// Drive the app's File > Open dialog over a folder of images
pub type AutomateFolder = Box<dyn Fn(&Path, &App) -> io::Result<()> + Sync>;

/// Tell the app to quit
pub type QuitApp = Box<dyn Fn(&App) -> io::Result<()> + Sync>;

impl JpegMini {
  pub fn live() -> Self {
    Self {
      is_installed: Box::new(|app| run_script(IS_INSTALLED, &[app.bundle_id]).map(|stdout| stdout == "true")),
      supports_assistive_devices: Box::new(|| run_script(SUPPORTS_ASSISTIVE_DEVICES, &[]).map(|stdout| stdout == "true")),
      automate_folder: Box::new(|dir, app| run_script(RUN_JPEGMINI, &[&dir.display().to_string(), app.name]).map(drop)),
      quit: Box::new(|app| run_script(QUIT_APP, &[app.name]).map(drop)),
    }
  }

  pub fn run(&self, files: &[PathBuf], fs: &dyn FileSystem, reporter: &Reporter) -> Result<(), ImageOptimError> {
    if files.is_empty() {
      return Ok(());
    }
    logger::debug("Locating JPEGmini installation");
    let app = self.find_installed_app()?.ok_or(ImageOptimError::JpegMiniNotInstalled)?;
    logger::debug("Checking support for assistive devices");
    if !(self.supports_assistive_devices)()? {
      return Err(ImageOptimError::AssistiveDevicesUnsupported);
    }
    reporter.on_app_start(app.name);
    let dir = symlink_dir_base().join(Uuid::new_v4().to_string());
    logger::debug(&format!("Linking {} images into {}", files.len(), dir.display()));
    // Registered before anything is created, so SIGINT mid-creation still
    // cleans up
    cleanup::register(&dir);
    let result = fs
      .create_symlink_dir(&dir, files)
      .and_then(|()| (self.automate_folder)(&dir, &app))
      .and_then(|()| {
        logger::debug(&format!("Quitting {}", app.name));
        (self.quit)(&app)
      })
      .map_err(ImageOptimError::from);
    let removed = fs.remove_dir(&dir);
    cleanup::deregister(&dir);
    result?;
    removed?;
    logger::debug(&format!("{} has finished", app.name));
    Ok(())
  }

  /// The first installed variant, None when none are, or the first error
  /// asking Finder
  fn find_installed_app(&self) -> Result<Option<App>, ImageOptimError> {
    JPEGMINI_VARIANTS
      .iter()
      .find_map(|app| match (self.is_installed)(app) {
        Ok(true) => Some(Ok(*app)),
        Ok(false) => None,
        Err(err) => Some(Err(err)),
      })
      .transpose()
      .map_err(ImageOptimError::from)
  }
}

#[cfg(test)]
#[path = "jpegmini_test.rs"]
mod jpegmini_test;
