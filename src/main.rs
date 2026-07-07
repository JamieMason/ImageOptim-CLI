mod apps;
mod cleanup;
mod cli;
mod discovery;
mod errors;
mod fs;
mod logger;
mod pipeline;
mod report;
mod reporter;
mod runners;
mod stats;
#[cfg(test)]
mod test;

use {
  crate::{
    cli::Cli,
    errors::{ISSUES_URL, ImageOptimError},
    fs::RealFileSystem,
    pipeline::Runners,
    reporter::Reporter,
    runners::{imagealpha::ImageAlpha, imageoptim::ImageOptim, jpegmini::JpegMini},
  },
  clap::error::ErrorKind,
  colored::Colorize,
  std::{env, process::exit},
};

fn main() {
  // Rust ignores SIGPIPE, which turns writes to a closed pipe (imageoptim |
  // head) into panics, and panic = "abort" in release turns those into
  // SIGABRT. Restore the conventional unix behaviour of dying quietly
  unsafe {
    libc::signal(libc::SIGPIPE, libc::SIG_DFL);
  }
  let cli = match Cli::parse_from_args(env::args_os()) {
    Ok(cli) => cli,
    Err(ImageOptimError::CliError(err)) if matches!(err.kind(), ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) => {
      let _ = err.print();
      exit(0);
    }
    Err(ImageOptimError::CliError(err)) => {
      let _ = err.print();
      exit(1);
    }
    Err(err) => {
      eprintln!("{}", logger::error(&err.to_string()));
      exit(1);
    }
  };
  logger::set_verbose(cli.verbose);
  if cli.no_color {
    colored::control::set_override(false);
  }
  let reporter = Reporter::new(cli.json);
  if let Err(err) = run(&cli, &reporter) {
    reporter.on_error(&err.to_string());
    eprintln!("{}", logger::error(&err.to_string()));
    if err.is_unexpected() {
      eprintln!();
      eprintln!("{}", logger::error(&format!("Please raise an issue at {}", ISSUES_URL.underline())));
    }
    exit(1);
  }
}

fn run(cli: &Cli, reporter: &Reporter) -> Result<(), ImageOptimError> {
  cli.validate()?;
  // Covers SIGINT, SIGTERM and SIGHUP via ctrlc's termination feature
  ctrlc::set_handler(|| {
    cleanup::remove_registered();
    exit(130);
  })
  .map_err(|err| std::io::Error::other(err.to_string()))?;
  let files = discovery::find_images(&cli.patterns, env::home_dir().as_deref())?;
  logger::debug(&format!("{} images matched", files.len()));
  if files.is_empty() {
    return Err(ImageOptimError::NoFilesMatched);
  }
  if cli.dry_run {
    for file in &files {
      reporter.on_match(file);
    }
    reporter.on_finished();
    return Ok(());
  }
  let runners = Runners {
    imagealpha: cli.imagealpha.then(ImageAlpha::live),
    imageoptim: (!cli.no_imageoptim).then(ImageOptim::live),
    jpegmini: cli.jpegmini.then(JpegMini::live),
  };
  pipeline::run(cli, &files, &RealFileSystem, &runners, reporter)
}
