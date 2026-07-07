use {assert_cmd::Command, predicates::prelude::*};

fn imageoptim() -> Command {
  Command::cargo_bin("imageoptim").unwrap()
}

#[test]
fn version_flag_prints_name_and_version() {
  imageoptim()
    .arg("--version")
    .assert()
    .success()
    .stdout(predicate::str::contains(format!("imageoptim {}", env!("CARGO_PKG_VERSION"))));
}

#[test]
fn help_flag_lists_every_option() {
  let assert = imageoptim().arg("--help").assert().success();
  let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  for option in [
    "--imagealpha",
    "--jpegmini",
    "--no-imageoptim",
    "--quality",
    "--speed",
    "--number-of-colors",
    "--batch-size",
    "--dry-run",
    "--json",
    "--verbose",
    "--no-stats",
    "--no-color",
  ] {
    assert!(stdout.contains(option), "--help is missing {option}\n{stdout}");
  }
}

#[test]
fn help_flag_lists_supported_apps_and_examples() {
  let assert = imageoptim().arg("--help").assert().success();
  let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  for line in ["Supported Apps:", "https://pngmini.com", "https://imageoptim.com", "Examples:"] {
    assert!(stdout.contains(line), "--help is missing {line}\n{stdout}");
  }
}

#[test]
fn rejects_unknown_flags() {
  imageoptim().arg("--wat").assert().code(1);
}

#[test]
fn rejects_quality_which_is_not_a_min_max_range() {
  imageoptim()
    .args(["--quality", "65"])
    .assert()
    .code(1)
    .stderr(predicate::str::contains("MIN-MAX"));
}

#[test]
fn rejects_quality_above_100() {
  imageoptim().args(["--quality", "65-101"]).assert().code(1);
}

#[test]
fn rejects_quality_where_min_exceeds_max() {
  imageoptim().args(["--quality", "80-65"]).assert().code(1);
}

#[test]
fn rejects_speed_of_zero() {
  imageoptim().args(["--speed", "0"]).assert().code(1);
}

#[test]
fn rejects_speed_above_ten() {
  imageoptim().args(["--speed", "11"]).assert().code(1);
}

#[test]
fn rejects_number_of_colors_below_two() {
  imageoptim().args(["--number-of-colors", "1"]).assert().code(1);
}

#[test]
fn rejects_number_of_colors_above_256() {
  imageoptim().args(["--number-of-colors", "257"]).assert().code(1);
}

#[test]
fn rejects_batch_size_of_zero() {
  imageoptim().args(["--batch-size", "0"]).assert().code(1);
}

#[test]
fn rejects_disabling_imageoptim_when_no_other_app_is_enabled() {
  imageoptim()
    .arg("--no-imageoptim")
    .assert()
    .code(1)
    .stderr(predicate::str::contains("No apps are enabled"));
}

#[test]
fn accepts_disabling_imageoptim_when_imagealpha_is_enabled() {
  let dir = tempfile::tempdir().unwrap();
  std::fs::write(dir.path().join("pixel.png"), "not-really-a-png").unwrap();
  imageoptim()
    .current_dir(dir.path())
    .args(["--no-imageoptim", "--imagealpha", "--dry-run"])
    .assert()
    .success();
}

#[test]
fn accepts_valid_quality_speed_and_colors() {
  let dir = tempfile::tempdir().unwrap();
  std::fs::write(dir.path().join("pixel.png"), "not-really-a-png").unwrap();
  imageoptim()
    .current_dir(dir.path())
    .args(["--dry-run", "--quality", "0-100", "--speed", "10", "--number-of-colors", "2"])
    .assert()
    .success();
}
