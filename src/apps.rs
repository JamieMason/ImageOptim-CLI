use std::{
  env,
  path::{Path, PathBuf},
};

pub const IMAGEALPHA_EXTENSIONS: [&str; 1] = ["png"];
pub const IMAGEOPTIM_EXTENSIONS: [&str; 9] = ["bmp", "gif", "jpeg", "jpg", "pcx", "png", "pnm", "tga", "tiff"];
pub const JPEGMINI_EXTENSIONS: [&str; 2] = ["jpeg", "jpg"];

/// Every extension supported by at least one app: ImageOptim's list is a
/// superset of ImageAlpha's (png) and JPEGmini's (jpeg, jpg)
pub const SUPPORTED_EXTENSIONS: [&str; 9] = IMAGEOPTIM_EXTENSIONS;

pub const IMAGEALPHA_URL: &str = "https://pngmini.com/";
pub const IMAGEOPTIM_URL: &str = "https://imageoptim.com/mac";
pub const JPEGMINI_URL: &str = "https://itunes.apple.com/us/app/jpegmini/id498944723";
pub const ASSISTIVE_DEVICES_URL: &str =
  "https://github.com/JamieMason/ImageOptim-CLI/#%EF%B8%8F-jpegmini-and-support-for-assistive-devices";

const PNGQUANT_BIN_PATH: &str = "/Applications/ImageAlpha.app/Contents/MacOS/pngquant";
const IMAGEOPTIM_BIN_PATH: &str = "/Applications/ImageOptim.app/Contents/MacOS/ImageOptim";
const OSASCRIPT_BIN_PATH: &str = "/usr/bin/osascript";

/// A JPEGmini app variant from the Mac App Store or sold directly
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct App {
  pub bundle_id: &'static str,
  pub name: &'static str,
}

/// Every known JPEGmini variant, in the order the TypeScript CLI checked for
/// them: the first installed variant wins
pub const JPEGMINI_VARIANTS: [App; 7] = [
  App {
    bundle_id: "com.icvt.JPEGmini-Pro",
    name: "JPEGmini Pro",
  },
  App {
    bundle_id: "com.icvt.JPEGmini-Pro-retail",
    name: "JPEGmini Pro",
  },
  App {
    bundle_id: "com.icvt.JPEGmini",
    name: "JPEGmini",
  },
  App {
    bundle_id: "com.icvt.JPEGmini-retail",
    name: "JPEGmini",
  },
  App {
    bundle_id: "com.icvt.JPEGminiLite",
    name: "JPEGmini Lite",
  },
  App {
    bundle_id: "com.icvt.JPEGminiLite-retail",
    name: "JPEGmini Lite",
  },
  App {
    bundle_id: "com.beamr.jpegminipro.app",
    name: "JPEGmini Pro",
  },
];

/// Overridable so that tests can substitute a fake pngquant
pub fn pngquant_path() -> PathBuf {
  path_from_env("IMAGEOPTIM_CLI_PNGQUANT_PATH", PNGQUANT_BIN_PATH)
}

/// Overridable so that tests can substitute a fake ImageOptim
pub fn imageoptim_path() -> PathBuf {
  path_from_env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", IMAGEOPTIM_BIN_PATH)
}

/// Overridable so that tests can substitute a fake osascript
pub fn osascript_path() -> PathBuf {
  path_from_env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", OSASCRIPT_BIN_PATH)
}

fn path_from_env(var: &str, default: &str) -> PathBuf {
  env::var_os(var).map(PathBuf::from).unwrap_or_else(|| PathBuf::from(default))
}

pub fn is_supported_extension(extensions: &[&str], path: &Path) -> bool {
  path
    .extension()
    .and_then(|extension| extension.to_str())
    .is_some_and(|extension| extensions.iter().any(|supported| supported.eq_ignore_ascii_case(extension)))
}

#[cfg(test)]
#[path = "apps_test.rs"]
mod apps_test;
