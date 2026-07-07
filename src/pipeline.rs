use {
  crate::{
    apps::{IMAGEALPHA_EXTENSIONS, JPEGMINI_EXTENSIONS, is_supported_extension},
    cli::Cli,
    errors::ImageOptimError,
    fs::FileSystem,
    logger,
    reporter::Reporter,
    runners::{imagealpha::ImageAlpha, imageoptim::ImageOptim, jpegmini::JpegMini},
    stats::{FileStat, Stats},
  },
  std::{path::PathBuf, thread},
};

/// The apps enabled by the CLI's flags
pub struct Runners {
  pub imagealpha: Option<ImageAlpha>,
  pub imageoptim: Option<ImageOptim>,
  pub jpegmini: Option<JpegMini>,
}

/// Process every batch of images, then confirm completion. Within a batch,
/// ImageAlpha and JPEGmini run in parallel threads, then ImageOptim runs over
/// everything, as the TypeScript CLI did
pub fn run(cli: &Cli, files: &[PathBuf], fs: &dyn FileSystem, runners: &Runners, reporter: &Reporter) -> Result<(), ImageOptimError> {
  reporter.on_start(files.len());
  let batch_count = files.len().div_ceil(cli.batch_size as usize);
  files.chunks(cli.batch_size as usize).enumerate().try_for_each(|(index, batch)| {
    logger::debug(&format!("Processing batch {} of {batch_count}", index + 1));
    process_batch(cli, batch, fs, runners, reporter)
  })?;
  reporter.on_finished();
  Ok(())
}

fn process_batch(cli: &Cli, batch: &[PathBuf], fs: &dyn FileSystem, runners: &Runners, reporter: &Reporter) -> Result<(), ImageOptimError> {
  let (batch, sizes_before): (Vec<PathBuf>, Vec<u64>) = existing_files(batch, fs)?.into_iter().unzip();
  let batch = batch.as_slice();
  if batch.is_empty() {
    return Ok(());
  }
  let pngs = only_supported(batch, &IMAGEALPHA_EXTENSIONS);
  let jpegs = only_supported(batch, &JPEGMINI_EXTENSIONS);
  let (imagealpha_result, jpegmini_result) = thread::scope(|scope| {
    let imagealpha = scope.spawn(|| match &runners.imagealpha {
      Some(imagealpha) if !pngs.is_empty() => {
        reporter.on_app_start("ImageAlpha");
        imagealpha.run(&pngs, &cli.quality, cli.speed, cli.number_of_colors)
      }
      _ => Ok(()),
    });
    let jpegmini = scope.spawn(|| match &runners.jpegmini {
      Some(jpegmini) => jpegmini.run(&jpegs, fs, reporter),
      None => Ok(()),
    });
    (imagealpha.join(), jpegmini.join())
  });
  imagealpha_result.expect("ImageAlpha thread panicked")?;
  jpegmini_result.expect("JPEGmini thread panicked")?;
  if let Some(imageoptim) = &runners.imageoptim {
    reporter.on_app_start("ImageOptim");
    imageoptim.run(batch)?;
  }
  let stats = batch
    .iter()
    .zip(&sizes_before)
    .filter_map(|(path, size_before)| match fs.file_size(path) {
      Ok(size_after) => Some(Ok(FileStat::new(path.display().to_string(), *size_before, size_after))),
      Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
        warn_vanished(path);
        None
      }
      Err(err) => Some(Err(ImageOptimError::from(err))),
    })
    .collect::<Result<Stats, ImageOptimError>>()?;
  if !cli.no_stats {
    reporter.on_batch_stats(&stats);
  }
  Ok(())
}

/// Files can be deleted by other programs between discovery and processing:
/// warn and carry on rather than aborting the whole run
fn existing_files(batch: &[PathBuf], fs: &dyn FileSystem) -> Result<Vec<(PathBuf, u64)>, ImageOptimError> {
  batch
    .iter()
    .filter_map(|path| match fs.file_size(path) {
      Ok(size) => Some(Ok((path.clone(), size))),
      Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
        warn_vanished(path);
        None
      }
      Err(err) => Some(Err(err.into())),
    })
    .collect()
}

fn warn_vanished(path: &std::path::Path) {
  eprintln!("{}", logger::warning(&format!("Skipped {}, it no longer exists", path.display())));
}

fn only_supported(batch: &[PathBuf], extensions: &[&str]) -> Vec<PathBuf> {
  batch
    .iter()
    .filter(|path| is_supported_extension(extensions, path))
    .cloned()
    .collect()
}

#[cfg(test)]
#[path = "pipeline_test.rs"]
mod pipeline_test;
