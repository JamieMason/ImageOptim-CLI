use super::*;

fn parse(args: &[&str]) -> Result<Cli, ImageOptimError> {
  Cli::parse_from_args(std::iter::once("imageoptim").chain(args.iter().copied()))
}

/// Batch size deviates from the TypeScript CLI's 300: batches no longer copy
/// files anywhere, so the old reason for small batches is gone and each batch
/// costs an ImageOptim.app launch and a JPEGmini GUI automation
#[test]
fn defaults_match_the_typescript_cli_except_batch_size() {
  let cli = parse(&[]).unwrap();
  assert_eq!(cli.quality, Quality { min: 65, max: 80 });
  assert_eq!(cli.speed, 1);
  assert_eq!(cli.number_of_colors, 256);
  assert_eq!(cli.batch_size, 3000);
  assert!(cli.patterns.is_empty());
  assert!(!cli.imagealpha);
  assert!(!cli.jpegmini);
  assert!(!cli.no_imageoptim);
  assert!(!cli.dry_run);
  assert!(!cli.json);
  assert!(!cli.verbose);
  assert!(!cli.no_stats);
  assert!(!cli.no_color);
}

#[test]
fn parses_a_valid_quality_range() {
  let cli = parse(&["--quality", "0-100"]).unwrap();
  assert_eq!(cli.quality, Quality { min: 0, max: 100 });
}

#[test]
fn rejects_quality_without_a_separator() {
  assert!(parse(&["--quality", "65"]).is_err());
}

#[test]
fn rejects_quality_where_min_exceeds_max() {
  assert!(parse(&["--quality", "80-65"]).is_err());
}

#[test]
fn rejects_quality_above_100() {
  assert!(parse(&["--quality", "0-101"]).is_err());
}

#[test]
fn rejects_disabling_every_app() {
  let cli = parse(&["--no-imageoptim"]).unwrap();
  assert!(matches!(cli.validate(), Err(ImageOptimError::NoAppsEnabled)));
}

#[test]
fn accepts_no_imageoptim_when_another_app_is_enabled() {
  assert!(parse(&["--no-imageoptim", "--imagealpha"]).unwrap().validate().is_ok());
  assert!(parse(&["--no-imageoptim", "--jpegmini"]).unwrap().validate().is_ok());
}

#[test]
fn accepts_short_flags() {
  let cli = parse(&["-a", "-j", "-I", "-S", "-C"]).unwrap();
  assert!(cli.imagealpha);
  assert!(cli.jpegmini);
  assert!(cli.no_imageoptim);
  assert!(cli.no_stats);
  assert!(cli.no_color);
}

#[test]
fn collects_positional_patterns() {
  let cli = parse(&["--imagealpha", "**/*.png", "~/Desktop"]).unwrap();
  assert_eq!(cli.patterns, vec!["**/*.png", "~/Desktop"]);
}
