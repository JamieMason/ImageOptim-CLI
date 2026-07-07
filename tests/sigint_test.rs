use {
  std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
  },
  tempfile::TempDir,
};

fn write(dir: &TempDir, relative_path: &str) {
  let path = dir.path().join(relative_path);
  fs::write(path, "fake-image-bytes").unwrap();
}

fn fake_bin(dir: &TempDir, name: &str, script: &str) -> PathBuf {
  let path = dir.path().join(name);
  fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
  fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
  path
}

fn wait_for(condition: impl Fn() -> bool, timeout: Duration) -> bool {
  let started = std::time::Instant::now();
  while started.elapsed() < timeout {
    if condition() {
      return true;
    }
    thread::sleep(Duration::from_millis(50));
  }
  false
}

#[test]
fn sigterm_removes_the_jpegmini_symlink_dir() {
  signal_removes_the_jpegmini_symlink_dir("-TERM");
}

#[test]
fn sigint_removes_the_jpegmini_symlink_dir() {
  signal_removes_the_jpegmini_symlink_dir("-INT");
}

fn signal_removes_the_jpegmini_symlink_dir(signal: &str) {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  let log = dir.path().join("jpegmini.txt");
  // The JPEGmini automation hangs, as if the app were still working, so that
  // this test can interrupt the CLI mid-run
  let osascript = fake_bin(
    &dir,
    "osascript",
    r#"script="$2"
case "$script" in
  *isInstalled*) echo true;;
  *"UI elements enabled"*) echo true;;
  *runJPEGmini*) echo "$3" >> "$FAKE_LOG"; sleep 30;;
esac"#,
  );
  let mut child = Command::new(env!("CARGO_BIN_EXE_imageoptim"))
    .current_dir(dir.path())
    .env("IMAGEOPTIM_CLI_OSASCRIPT_PATH", &osascript)
    .env("FAKE_LOG", &log)
    .args(["--jpegmini", "--no-imageoptim"])
    .spawn()
    .unwrap();

  assert!(wait_for(|| log.exists(), Duration::from_secs(10)), "automation never started");
  let symlink_dir = PathBuf::from(fs::read_to_string(&log).unwrap().lines().next().unwrap().to_string());
  assert!(symlink_dir.exists(), "symlink dir was never created");

  Command::new("kill").args([signal, &child.id().to_string()]).status().unwrap();

  assert!(
    wait_for(|| !Path::exists(&symlink_dir), Duration::from_secs(10)),
    "symlink dir was not cleaned up after SIGINT"
  );
  let status = child.wait().unwrap();
  assert_eq!(status.code(), Some(130));
}
