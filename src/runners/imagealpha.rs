use {
  crate::{
    apps,
    cli::Quality,
    errors::ImageOptimError,
    logger,
    runners::{BinaryExists, ExecOutcome, exec},
  },
  std::{ffi::OsString, io, path::PathBuf, thread},
};

/// pngquant exits with 99 when no file could be saved within the quality range
const QUALITY_UNREACHABLE: i32 = 99;
/// pngquant exits with 98 when --skip-if-larger rejects every result
const SKIPPED_LARGER_FILES: i32 = 98;

/// Run pngquant with the given arguments
pub type RunPngquant = Box<dyn Fn(&[OsString]) -> io::Result<ExecOutcome> + Sync>;

/// Runs ImageAlpha.app's bundled pngquant over PNG files in place. pngquant
/// compresses its file list sequentially on one core, so the files are spread
/// over one pngquant process per core
pub struct ImageAlpha {
  pub binary_exists: BinaryExists,
  pub run_pngquant: RunPngquant,
  pub max_shards: usize,
}

impl ImageAlpha {
  pub fn live() -> Self {
    Self {
      binary_exists: Box::new(|| apps::pngquant_path().exists()),
      run_pngquant: Box::new(|args| {
        let program = apps::pngquant_path();
        exec(program.as_os_str(), &args.iter().map(OsString::as_os_str).collect::<Vec<_>>())
      }),
      max_shards: thread::available_parallelism().map(usize::from).unwrap_or(1),
    }
  }

  pub fn run(&self, files: &[PathBuf], quality: &Quality, speed: u8, number_of_colors: u16) -> Result<(), ImageOptimError> {
    if files.is_empty() {
      return Ok(());
    }
    if !(self.binary_exists)() {
      return Err(ImageOptimError::ImageAlphaNotInstalled);
    }
    let shard_size = files.len().div_ceil(self.max_shards.max(1));
    thread::scope(|scope| {
      files
        .chunks(shard_size)
        .map(|shard| scope.spawn(|| self.run_shard(shard, quality, speed, number_of_colors)))
        // Spawn every shard before joining any, so they run in parallel
        .collect::<Vec<_>>()
        .into_iter()
        .try_for_each(|handle| handle.join().expect("pngquant shard thread panicked"))
    })?;
    logger::debug("ImageAlpha has finished");
    Ok(())
  }

  fn run_shard(&self, files: &[PathBuf], quality: &Quality, speed: u8, number_of_colors: u16) -> Result<(), ImageOptimError> {
    let mut args: Vec<OsString> = vec![
      "--ext=.png".into(),
      "--force".into(),
      "--skip-if-larger".into(),
      format!("--speed={speed}").into(),
      format!("--quality={quality}").into(),
      number_of_colors.to_string().into(),
      "--".into(),
    ];
    args.extend(files.iter().map(|file| file.clone().into_os_string()));
    let outcome = (self.run_pngquant)(&args)?;
    match outcome.status {
      Some(0 | QUALITY_UNREACHABLE | SKIPPED_LARGER_FILES) => Ok(()),
      status => Err(ImageOptimError::app_failed("pngquant", status.unwrap_or(-1), &outcome.stderr)),
    }
  }
}

#[cfg(test)]
#[path = "imagealpha_test.rs"]
mod imagealpha_test;
