use {
  assert_cmd::Command,
  serde_json::Value,
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

fn fake_bin(dir: &TempDir, name: &str, script: &str) -> PathBuf {
  let path = dir.path().join(name);
  fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
  fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
  path
}

/// Every stdout line must be valid JSON: parse them all
fn ndjson_lines(stdout: &[u8]) -> Vec<Value> {
  String::from_utf8(stdout.to_vec())
    .unwrap()
    .lines()
    .map(|line| serde_json::from_str(line).unwrap_or_else(|_| panic!("stdout line is not JSON: {line}")))
    .collect()
}

#[test]
fn json_dry_run_streams_match_events_then_done() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  write(&dir, "photo.jpg");
  let assert = imageoptim(&dir).args(["--dry-run", "--json"]).assert().success();
  let lines = ndjson_lines(&assert.get_output().stdout);
  assert_eq!(lines[0], serde_json::json!({ "type": "match", "path": "photo.jpg" }));
  assert_eq!(lines[1], serde_json::json!({ "type": "match", "path": "pixel.png" }));
  assert_eq!(lines[2], serde_json::json!({ "type": "done" }));
}

#[test]
fn json_run_streams_progress_files_and_totals() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", r#"for f in "$@"; do printf 'tiny' > "$f"; done"#);
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .arg("--json")
    .assert()
    .success();
  let lines = ndjson_lines(&assert.get_output().stdout);
  assert_eq!(
    lines[0],
    serde_json::json!({ "type": "start", "version": env!("CARGO_PKG_VERSION"), "files": 1 })
  );
  assert_eq!(lines[1], serde_json::json!({ "type": "app_start", "app": "ImageOptim" }));
  assert_eq!(
    lines[2],
    serde_json::json!({ "type": "file", "path": "pixel.png", "before": 16, "after": 4, "saving": 12, "percent": 75.0 })
  );
  assert_eq!(
    lines[3],
    serde_json::json!({ "type": "total", "before": 16, "after": 4, "saving": 12, "percent": 75.0 })
  );
  assert_eq!(lines[4], serde_json::json!({ "type": "done" }));
}

#[test]
fn json_errors_are_events_on_stdout() {
  let dir = TempDir::new().unwrap();
  write(&dir, "notes.txt");
  let assert = imageoptim(&dir).args(["--dry-run", "--json"]).assert().code(1);
  let lines = ndjson_lines(&assert.get_output().stdout);
  assert_eq!(
    lines[0],
    serde_json::json!({ "type": "error", "message": "No images matched the patterns provided" })
  );
}

#[test]
fn json_mode_keeps_human_output_off_stdout() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", "exit 0");
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .arg("--json")
    .assert()
    .success();
  let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  assert!(!stdout.contains("Finished"), "human output leaked into NDJSON:\n{stdout}");
  ndjson_lines(&assert.get_output().stdout);
}

#[test]
fn verbose_flag_prints_debug_lines_to_stderr() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", "exit 0");
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .arg("--verbose")
    .assert()
    .success();
  let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
  assert!(stderr.contains("? 1 images matched"), "missing verbose match count:\n{stderr}");
  assert!(stderr.contains("? exec"), "missing verbose exec line:\n{stderr}");
}

#[test]
fn without_verbose_flag_there_are_no_debug_lines() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", "exit 0");
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .assert()
    .success();
  let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
  assert!(!stderr.contains("? "), "unexpected verbose output:\n{stderr}");
}

#[test]
fn json_no_apps_enabled_is_an_error_event() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let assert = imageoptim(&dir).args(["--json", "--no-imageoptim"]).assert().code(1);
  let lines = ndjson_lines(&assert.get_output().stdout);
  assert_eq!(lines[0]["type"], "error");
  assert!(
    lines[0]["message"].as_str().unwrap().contains("No apps are enabled"),
    "{}",
    lines[0]
  );
}

#[test]
fn json_does_not_announce_apps_which_have_nothing_to_do() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", "exit 0");
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .env("IMAGEOPTIM_CLI_PNGQUANT_PATH", "/nonexistent/pngquant")
    .args(["--json", "--imagealpha"])
    .assert()
    .success();
  let lines = ndjson_lines(&assert.get_output().stdout);
  assert!(
    !lines.iter().any(|line| line["type"] == "app_start" && line["app"] == "ImageAlpha"),
    "ImageAlpha announced with no PNGs to process: {lines:?}"
  );
}

/// The fake osascript answers as an installed JPEGmini Pro
fn fake_osascript(dir: &TempDir) -> PathBuf {
  fake_bin(
    dir,
    "osascript",
    r#"script="$2"
case "$script" in
  *isInstalled*) echo true;;
  *"UI elements enabled"*) echo true;;
esac"#,
  )
}

#[test]
fn json_announces_the_detected_jpegmini_variant() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let osascript = fake_osascript(&dir);
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .args(["--json", "--jpegmini", "--no-imageoptim"])
    .assert()
    .success();
  let lines = ndjson_lines(&assert.get_output().stdout);
  assert!(
    lines
      .iter()
      .any(|line| line == &serde_json::json!({ "type": "app_start", "app": "JPEGmini Pro" })),
    "missing JPEGmini app_start: {lines:?}"
  );
}

#[test]
fn json_streams_each_batch_as_it_completes() {
  let dir = TempDir::new().unwrap();
  write(&dir, "a.png");
  write(&dir, "b.png");
  let imageoptim_bin = fake_bin(&dir, "ImageOptim", r#"for f in "$@"; do printf 'tiny' > "$f"; done"#);
  let assert = imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .args(["--json", "--batch-size", "1"])
    .assert()
    .success();
  let lines = ndjson_lines(&assert.get_output().stdout);
  let types: Vec<&str> = lines.iter().map(|line| line["type"].as_str().unwrap()).collect();
  assert_eq!(
    types,
    ["start", "app_start", "file", "total", "app_start", "file", "total", "done"],
    "each batch must stream its own rows and totals: {lines:?}"
  );
  assert_eq!(lines[2]["path"], "a.png");
  assert_eq!(lines[5]["path"], "b.png");
}
