use {
  crate::apps::{ASSISTIVE_DEVICES_URL, IMAGEALPHA_URL, IMAGEOPTIM_URL, JPEGMINI_URL},
  thiserror::Error,
};

pub const ISSUES_URL: &str = "https://github.com/JamieMason/ImageOptim-CLI/issues";

#[derive(Debug, Error)]
pub enum ImageOptimError {
  #[error("{0}")]
  CliError(#[from] clap::Error),
  #[error("No apps are enabled, remove --no-imageoptim or also pass --imagealpha or --jpegmini")]
  NoAppsEnabled,
  #[error("No images matched the patterns provided")]
  NoFilesMatched,
  #[error("Invalid pattern '{pattern}': {reason}")]
  InvalidPattern { pattern: String, reason: String },
  #[error("Unsupported pattern '{pattern}': {reason}")]
  UnsupportedPattern { pattern: String, reason: String },
  #[error("Failed to read {path}: {reason}")]
  UnreadablePath { path: String, reason: String },
  #[error("ImageAlpha.app is not installed ({IMAGEALPHA_URL})")]
  ImageAlphaNotInstalled,
  #[error("ImageOptim.app is not installed ({IMAGEOPTIM_URL})")]
  ImageOptimNotInstalled,
  #[error("JPEGmini is not installed ({JPEGMINI_URL})")]
  JpegMiniNotInstalled,
  #[error("Support for assistive devices needed, see {ASSISTIVE_DEVICES_URL}")]
  AssistiveDevicesUnsupported,
  #[error("{app} exited with status {status}{stderr}")]
  AppFailed { app: &'static str, status: i32, stderr: String },
  #[error("{0}")]
  IoError(#[from] std::io::Error),
}

impl ImageOptimError {
  pub fn app_failed(app: &'static str, status: i32, stderr: &str) -> Self {
    let stderr = stderr.trim();
    Self::AppFailed {
      app,
      status,
      stderr: if stderr.is_empty() { String::new() } else { format!(": {stderr}") },
    }
  }

  /// Errors which are bugs rather than expected states the user can correct,
  /// where the TypeScript CLI asked for an issue to be raised
  pub fn is_unexpected(&self) -> bool {
    matches!(self, Self::IoError(_) | Self::AppFailed { .. })
  }
}
