use {
  assert_cmd::Command,
  predicates::prelude::*,
  std::{fs, os::unix::fs::PermissionsExt, path::PathBuf},
  tempfile::TempDir,
};

fn imageoptim(dir: &TempDir) -> Command {
  let mut command = Command::cargo_bin("imageoptim").unwrap();
  command.current_dir(dir.path());
  command
}

fn write(dir: &TempDir, relative_path: &str) {
  let path = dir.path().join(relative_path);
  fs::create_dir_all(path.parent().unwrap()).unwrap();
  fs::write(path, "fake-image-bytes").unwrap();
}

/// Create an executable shell script which stands in for pngquant, the
/// ImageOptim binary, or osascript
fn fake_bin(dir: &TempDir, name: &str, script: &str) -> PathBuf {
  let path = dir.path().join(name);
  fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
  fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
  path
}

/// A fake osascript which answers the 4 embedded applescripts: whether an app
/// bundle id is installed, whether assistive devices are supported, the
/// JPEGmini folder automation, and quitting an app
fn fake_osascript(dir: &TempDir) -> PathBuf {
  fake_bin(
    dir,
    "osascript",
    r#"script="$2"
case "$script" in
  *isInstalled*) case "$3" in ${FAKE_INSTALLED_BUNDLE_ID:-none}) echo true;; *) echo false;; esac;;
  *"UI elements enabled"*) echo "${FAKE_ASSISTIVE:-true}";;
  *runJPEGmini*) echo "$3" >> "${FAKE_LOG:-/dev/null}"; ls -RA "$3" >> "${FAKE_LOG:-/dev/null}";;
  *quitApp*) echo "quit $3" >> "${FAKE_LOG:-/dev/null}";;
esac"#,
  )
}

#[test]
fn errors_when_imagealpha_is_not_installed() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", "/nonexistent/pngquant")
    .args(["--imagealpha", "--no-imageoptim"])
    .assert()
    .code(1)
    .stderr(predicate::str::contains("ImageAlpha.app is not installed (https://pngmini.com/)"));
}

#[test]
fn errors_when_imageoptim_is_not_installed() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", "/nonexistent/ImageOptim")
    .assert()
    .code(1)
    .stderr(predicate::str::contains(
      "ImageOptim.app is not installed (https://imageoptim.com/mac)",
    ));
}

#[test]
fn runs_pngquant_over_pngs_with_the_expected_arguments() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  write(&dir, "photo.jpg");
  let log = dir.path().join("args.txt");
  let pngquant = fake_bin(&dir, "pngquant", r#"printf '%s\n' "$@" > "$FAKE_LOG""#);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", &pngquant)
    .env("FAKE_LOG", &log)
    .args(["--imagealpha", "--no-imageoptim"])
    .assert()
    .success();
  let args = fs::read_to_string(&log).unwrap();
  assert_eq!(
    args,
    "--ext=.png\n--force\n--skip-if-larger\n--speed=1\n--quality=65-80\n256\n--\npixel.png\n"
  );
}

#[test]
fn runs_pngquant_with_custom_quality_speed_and_colors() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let log = dir.path().join("args.txt");
  let pngquant = fake_bin(&dir, "pngquant", r#"printf '%s\n' "$@" > "$FAKE_LOG""#);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", &pngquant)
    .env("FAKE_LOG", &log)
    .args([
      "--imagealpha",
      "--no-imageoptim",
      "--quality",
      "10-20",
      "--speed",
      "3",
      "--number-of-colors",
      "128",
    ])
    .assert()
    .success();
  let args = fs::read_to_string(&log).unwrap();
  assert_eq!(
    args,
    "--ext=.png\n--force\n--skip-if-larger\n--speed=3\n--quality=10-20\n128\n--\npixel.png\n"
  );
}

#[test]
fn tolerates_pngquant_exit_codes_98_and_99() {
  for code in [98, 99] {
    let dir = TempDir::new().unwrap();
    write(&dir, "pixel.png");
    let pngquant = fake_bin(&dir, "pngquant", &format!("exit {code}"));
    imageoptim(&dir)
      .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", &pngquant)
      .args(["--imagealpha", "--no-imageoptim"])
      .assert()
      .success()
      .stdout(predicate::str::contains("Finished"));
  }
}

#[test]
fn fails_on_any_other_pngquant_exit_code() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let pngquant = fake_bin(&dir, "pngquant", "echo 'error: broken' >&2\nexit 2");
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", &pngquant)
    .args(["--imagealpha", "--no-imageoptim"])
    .assert()
    .code(1)
    .stderr(predicate::str::contains("pngquant"));
}

#[test]
fn runs_the_imageoptim_binary_over_every_supported_file() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  write(&dir, "photo.jpg");
  write(&dir, "notes.txt");
  let log = dir.path().join("args.txt");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", r#"printf '%s\n' "$@" > "$FAKE_LOG""#);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .env("FAKE_LOG", &log)
    .assert()
    .success();
  let args = fs::read_to_string(&log).unwrap();
  assert_eq!(args, "photo.jpg\npixel.png\n");
}

#[test]
fn reports_size_savings_made_by_the_apps() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", r#"for f in "$@"; do printf 'tiny' > "$f"; done"#);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .assert()
    .success()
    .stdout(predicate::str::contains("pixel.png was: 16B now: 4B saving: 12B (75.00%)"))
    .stdout(predicate::str::contains("TOTAL was: 16B now: 4B saving: 12B (75.00%)"))
    .stdout(predicate::str::contains("Finished"));
}

#[test]
fn errors_when_jpegmini_is_not_installed() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let osascript = fake_osascript(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "none")
    .args(["--jpegmini", "--no-imageoptim"])
    .assert()
    .code(1)
    .stderr(predicate::str::contains(
      "JPEGmini is not installed (https://itunes.apple.com/us/app/jpegmini/id498944723)",
    ));
}

#[test]
fn errors_when_assistive_devices_are_not_supported() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let osascript = fake_osascript(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "com.icvt.JPEGmini-Pro")
    .env("FAKE_ASSISTIVE", "ERROR_GUISCRIPT_UNREADABLE")
    .args(["--jpegmini", "--no-imageoptim"])
    .assert()
    .code(1)
    .stderr(predicate::str::contains("Support for assistive devices needed"));
}

#[test]
fn runs_jpegmini_over_a_directory_of_symlinks_to_the_jpegs() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  write(&dir, "pixel.png");
  let log = dir.path().join("jpegmini.txt");
  let osascript = fake_osascript(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "com.icvt.JPEGmini-Pro")
    .env("FAKE_LOG", &log)
    .args(["--jpegmini", "--no-imageoptim"])
    .assert()
    .success()
    .stderr(predicate::str::contains("Running JPEGmini Pro"));
  let log = fs::read_to_string(&log).unwrap();
  assert!(log.contains("photo.jpg"), "JPEGmini did not see photo.jpg:\n{log}");
  assert!(!log.contains("pixel.png"), "JPEGmini must only see jpegs:\n{log}");
  assert!(log.contains("quit JPEGmini Pro"), "JPEGmini was not quit:\n{log}");
}

#[test]
fn detects_which_jpegmini_variant_is_installed() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let osascript = fake_osascript(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "com.icvt.JPEGminiLite")
    .args(["--jpegmini", "--no-imageoptim"])
    .assert()
    .success()
    .stderr(predicate::str::contains("Running JPEGmini Lite"));
}

#[test]
fn cleans_up_the_symlink_directory_after_running() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let log = dir.path().join("jpegmini.txt");
  let osascript = fake_osascript(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "com.icvt.JPEGmini-Pro")
    .env("FAKE_LOG", &log)
    .args(["--jpegmini", "--no-imageoptim"])
    .assert()
    .success();
  let log = fs::read_to_string(&log).unwrap();
  let symlink_dir = log.lines().next().unwrap();
  let expected_base = std::env::temp_dir().join("imageoptim-cli");
  assert!(
    PathBuf::from(symlink_dir).starts_with(&expected_base),
    "unexpected dir: {symlink_dir}, expected under {}",
    expected_base.display()
  );
  assert!(!PathBuf::from(symlink_dir).exists(), "{symlink_dir} was not removed");
}

#[test]
fn original_files_remain_in_place_after_jpegmini_runs() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let osascript = fake_osascript(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "com.icvt.JPEGmini-Pro")
    .args(["--jpegmini", "--no-imageoptim"])
    .assert()
    .success();
  assert_eq!(fs::read_to_string(dir.path().join("photo.jpg")).unwrap(), "fake-image-bytes");
}

#[test]
fn no_stats_suppresses_the_report_but_still_finishes() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", r#"for f in "$@"; do printf 'tiny' > "$f"; done"#);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .arg("--no-stats")
    .assert()
    .success()
    .stdout(predicate::str::contains("was:").not())
    .stdout(predicate::str::contains("TOTAL").not())
    .stdout(predicate::str::contains("Finished"));
}

#[test]
fn verbose_logs_the_pngquant_and_osascript_invocations() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  write(&dir, "photo.jpg");
  let pngquant = fake_bin(&dir, "pngquant", "exit 0");
  let osascript = fake_osascript(&dir);
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", &pngquant)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_INSTALLED_BUNDLE_ID", "com.icvt.JPEGmini-Pro")
    .args(["--verbose", "--imagealpha", "--jpegmini", "--no-imageoptim"])
    .assert()
    .success();
  let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
  assert!(
    stderr.contains("? exec") && stderr.contains("pngquant"),
    "missing pngquant exec line:\n{stderr}"
  );
  assert!(stderr.contains("osascript"), "missing osascript exec line:\n{stderr}");
}
