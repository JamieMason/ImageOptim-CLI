use std::{fs, path::PathBuf, process::Command};

fn scripts_dir() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/runners/osascript")
}

#[test]
fn every_applescript_compiles() {
  let out_dir = tempfile::tempdir().unwrap();
  let mut checked = 0;
  for entry in fs::read_dir(scripts_dir()).unwrap() {
    let path = entry.unwrap().path();
    if path.extension().is_none_or(|extension| extension != "applescript") {
      continue;
    }
    let compiled = out_dir.path().join("compiled.scpt");
    let source = fs::read_to_string(&path).unwrap();
    let output = Command::new("/usr/bin/osacompile")
      .args(["-e", &source, "-o"])
      .arg(&compiled)
      .output()
      .unwrap();
    assert!(
      output.status.success(),
      "{} failed to compile:\n{}",
      path.display(),
      String::from_utf8_lossy(&output.stderr)
    );
    checked += 1;
  }
  assert_eq!(checked, 4, "expected to compile all 4 embedded applescripts");
}

#[test]
fn jpegmini_automation_polls_instead_of_sleeping() {
  let script = fs::read_to_string(scripts_dir().join("run-jpegmini.applescript")).unwrap();
  for line in script.lines().filter(|line| line.trim_start().starts_with("delay")) {
    assert!(
      line.contains("delay 0."),
      "fixed sleep remains in run-jpegmini.applescript: {line:?}"
    );
  }
}

/// Runs the real /usr/bin/osascript, proving the -e invocation style, argv
/// plumbing, and Finder bundle-id query all work outside the fakes
#[test]
fn is_installed_answers_via_the_real_osascript() {
  let script = fs::read_to_string(scripts_dir().join("is-installed.applescript")).unwrap();
  let ask = |bundle_id: &str| {
    let output = Command::new("/usr/bin/osascript")
      .args(["-e", &script, bundle_id])
      .output()
      .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().trim().to_string()
  };
  assert_eq!(ask("com.apple.finder"), "true", "Finder is installed on every Mac");
  assert_eq!(ask("com.example.definitely-not-installed"), "false");
}
