use {
  crate::errors::ImageOptimError,
  clap::Parser,
  std::fmt::{self, Display},
};

const AFTER_HELP: &str = "Supported Apps:

  ImageAlpha: https://pngmini.com
  ImageOptim: https://imageoptim.com
  JPEGmini Lite: https://itunes.apple.com/us/app/jpegmini-lite/id525742250
  JPEGmini Pro: https://itunes.apple.com/us/app/jpegmini-pro/id887163276
  JPEGmini: https://itunes.apple.com/us/app/jpegmini/id498944723

Examples:

  Run ImageOptim.app over every image in current directory
  imageoptim

  Run ImageAlpha.app and ImageOptim.app over every PNG in current directory
  imageoptim --imagealpha '**/*.png'

  Run JPEGmini.app and ImageOptim.app over every JPG in current directory
  imageoptim --jpegmini '**/*.jpg' '**/*.jpeg'

  Run JPEGmini.app over every JPG in current directory
  imageoptim --jpegmini --no-imageoptim '**/*.jpg' '**/*.jpeg'

  Run ImageOptim.app over every image in a specific directory
  imageoptim '~/Desktop'";

/// An ImageAlpha quality range such as 65-80, where both ends are 0-100
#[derive(Clone, Debug, PartialEq)]
pub struct Quality {
  pub min: u8,
  pub max: u8,
}

impl Display for Quality {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}-{}", self.min, self.max)
  }
}

fn parse_quality(value: &str) -> Result<Quality, String> {
  let err = || format!("'{value}' must be MIN-MAX within 0-100, such as 65-80");
  let (min, max) = value.split_once('-').ok_or_else(err)?;
  let min = min.trim().parse::<u8>().map_err(|_| err())?;
  let max = max.trim().parse::<u8>().map_err(|_| err())?;
  if min > max || max > 100 {
    return Err(err());
  }
  Ok(Quality { min, max })
}

#[derive(Debug, Parser)]
#[command(name = "imageoptim", version, about, after_help = AFTER_HELP)]
pub struct Cli {
  /// Glob patterns of images or directories to optimise
  #[arg(value_name = "PATTERNS")]
  pub patterns: Vec<String>,

  /// Enable ImageAlpha
  #[arg(short = 'a', long)]
  pub imagealpha: bool,

  /// Enable JPEGmini
  #[arg(short = 'j', long)]
  pub jpegmini: bool,

  /// Disable ImageOptim
  #[arg(short = 'I', long)]
  pub no_imageoptim: bool,

  /// ImageAlpha quality range from 0-100
  #[arg(long, value_name = "MIN-MAX", default_value = "65-80", value_parser = parse_quality)]
  pub quality: Quality,

  /// ImageAlpha speed from 1 (brute-force) to 10 (fastest)
  #[arg(long, value_name = "N", default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
  pub speed: u8,

  /// ImageAlpha palette size from 2-256
  #[arg(long, value_name = "N", default_value_t = 256, value_parser = clap::value_parser!(u16).range(2..=256))]
  pub number_of_colors: u16,

  /// How many images to process at a time
  #[arg(long, value_name = "N", default_value_t = 3000, value_parser = clap::value_parser!(u32).range(1..))]
  pub batch_size: u32,

  /// List images which would be optimised, without optimising them
  #[arg(long)]
  pub dry_run: bool,

  /// Output newline-delimited JSON instead of human-readable text
  #[arg(long)]
  pub json: bool,

  /// Output debug logging
  #[arg(long)]
  pub verbose: bool,

  /// Do not display file size savings and quality loss information
  #[arg(short = 'S', long)]
  pub no_stats: bool,

  /// Output to the terminal without colors
  #[arg(short = 'C', long)]
  pub no_color: bool,
}

impl Cli {
  pub fn parse_from_args<I, T>(args: I) -> Result<Self, ImageOptimError>
  where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
  {
    Ok(Cli::try_parse_from(args)?)
  }

  /// Checked after the Reporter exists, so that with --json this failure is
  /// emitted as an NDJSON error event like every other runtime error
  pub fn validate(&self) -> Result<(), ImageOptimError> {
    if self.no_imageoptim && !self.imagealpha && !self.jpegmini {
      return Err(ImageOptimError::NoAppsEnabled);
    }
    Ok(())
  }
}

#[cfg(test)]
#[path = "cli_test.rs"]
mod cli_test;
