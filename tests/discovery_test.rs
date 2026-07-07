use {
  assert_cmd::Command,
  predicates::prelude::*,
  std::{fs, os::unix::fs::PermissionsExt},
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

fn fake_imageoptim_bin(dir: &TempDir) -> std::path::PathBuf {
  let path = dir.path().join("ImageOptim");
  fs::write(&path, "#!/bin/sh\nexit 0\n").unwrap();
  fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
  path
}

#[test]
fn dry_run_lists_only_supported_image_types() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  write(&dir, "photo.jpg");
  write(&dir, "notes.txt");
  write(&dir, "script.js");
  imageoptim(&dir)
    .arg("--dry-run")
    .assert()
    .success()
    .stdout(predicate::str::contains("pixel.png"))
    .stdout(predicate::str::contains("photo.jpg"))
    .stdout(predicate::str::contains("notes.txt").not())
    .stdout(predicate::str::contains("script.js").not());
}

#[test]
fn dry_run_finds_images_recursively_when_no_patterns_given() {
  let dir = TempDir::new().unwrap();
  write(&dir, "deeply/nested/dirs/pixel.png");
  imageoptim(&dir)
    .arg("--dry-run")
    .assert()
    .success()
    .stdout(predicate::str::contains("deeply/nested/dirs/pixel.png"));
}

#[test]
fn dry_run_supports_every_imageoptim_file_type() {
  let dir = TempDir::new().unwrap();
  let file_names = ["a.bmp", "b.gif", "c.jpeg", "d.jpg", "e.pcx", "f.png", "g.pnm", "h.tga", "i.tiff"];
  for file_name in file_names {
    write(&dir, file_name);
  }
  let assert = imageoptim(&dir).arg("--dry-run").assert().success();
  let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  for file_name in file_names {
    assert!(stdout.contains(file_name), "missing {file_name}\n{stdout}");
  }
}

#[test]
fn dry_run_matches_uppercase_extensions() {
  let dir = TempDir::new().unwrap();
  write(&dir, "PIXEL.PNG");
  write(&dir, "PHOTO.JPG");
  imageoptim(&dir)
    .arg("--dry-run")
    .assert()
    .success()
    .stdout(predicate::str::contains("PIXEL.PNG"))
    .stdout(predicate::str::contains("PHOTO.JPG"));
}

#[test]
fn dry_run_respects_explicit_glob_patterns() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  write(&dir, "photo.jpg");
  imageoptim(&dir)
    .args(["--dry-run", "*.png"])
    .assert()
    .success()
    .stdout(predicate::str::contains("pixel.png"))
    .stdout(predicate::str::contains("photo.jpg").not());
}

#[test]
fn dry_run_expands_a_directory_pattern_to_the_images_inside_it() {
  let dir = TempDir::new().unwrap();
  write(&dir, "images/logos/brand.png");
  write(&dir, "elsewhere/other.png");
  imageoptim(&dir)
    .args(["--dry-run", "images"])
    .assert()
    .success()
    .stdout(predicate::str::contains("images/logos/brand.png"))
    .stdout(predicate::str::contains("elsewhere/other.png").not());
}

#[test]
fn dry_run_ignores_hidden_files_by_default() {
  let dir = TempDir::new().unwrap();
  write(&dir, "visible.png");
  write(&dir, ".hidden.png");
  write(&dir, ".hidden-dir/pixel.png");
  imageoptim(&dir)
    .arg("--dry-run")
    .assert()
    .success()
    .stdout(predicate::str::contains("visible.png"))
    .stdout(predicate::str::contains("hidden").not());
}

#[test]
fn dry_run_deduplicates_files_matched_by_overlapping_patterns() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let assert = imageoptim(&dir).args(["--dry-run", "*.png", "**/*"]).assert().success();
  let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  assert_eq!(stdout.matches("pixel.png").count(), 1, "{stdout}");
}

#[test]
fn exits_1_when_no_images_match() {
  let dir = TempDir::new().unwrap();
  write(&dir, "notes.txt");
  imageoptim(&dir)
    .arg("--dry-run")
    .assert()
    .code(1)
    .stderr(predicate::str::contains("No images matched the patterns provided"));
}

#[test]
fn without_dry_run_reports_stats_and_finishes() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  let imageoptim_bin = fake_imageoptim_bin(&dir);
  imageoptim(&dir)
    .env("IMAGEOPTIM_CLI_IMAGEOPTIM_PATH", &imageoptim_bin)
    .assert()
    .success()
    .stdout(predicate::str::contains("No size savings"))
    .stdout(predicate::str::contains("Finished"));
}

#[test]
fn negation_patterns_exclude_files_like_globby_did() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  write(&dir, "originals/master.jpg");
  imageoptim(&dir)
    .args(["--dry-run", "**/*.jpg", "!originals/**"])
    .assert()
    .success()
    .stdout(predicate::str::contains("photo.jpg"))
    .stdout(predicate::str::contains("master.jpg").not());
}

#[test]
fn negation_patterns_apply_to_the_default_pattern() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photo.jpg");
  write(&dir, "originals/master.jpg");
  imageoptim(&dir)
    .args(["--dry-run", "!originals/**"])
    .assert()
    .success()
    .stdout(predicate::str::contains("photo.jpg"))
    .stdout(predicate::str::contains("master.jpg").not());
}

#[test]
fn directory_names_containing_glob_metacharacters_still_match() {
  let dir = TempDir::new().unwrap();
  write(&dir, "Photos [2024]/pixel.png");
  imageoptim(&dir)
    .args(["--dry-run", "Photos [2024]"])
    .assert()
    .success()
    .stdout(predicate::str::contains("pixel.png"));
}

#[test]
fn the_same_file_via_different_spellings_is_deduplicated() {
  let dir = TempDir::new().unwrap();
  write(&dir, "photos/hero.png");
  let assert = imageoptim(&dir)
    .args(["--dry-run", "./photos", "photos/hero.png"])
    .assert()
    .success();
  let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
  assert_eq!(stdout.matches("hero.png").count(), 1, "{stdout}");
}

#[test]
fn star_does_not_cross_directory_boundaries() {
  let dir = TempDir::new().unwrap();
  write(&dir, "top.png");
  write(&dir, "nested/deep.png");
  imageoptim(&dir)
    .args(["--dry-run", "*.png"])
    .assert()
    .success()
    .stdout(predicate::str::contains("top.png"))
    .stdout(predicate::str::contains("deep.png").not());
}

#[test]
fn explicitly_named_hidden_files_are_matched() {
  let dir = TempDir::new().unwrap();
  write(&dir, ".hidden.png");
  imageoptim(&dir)
    .args(["--dry-run", ".hidden.png"])
    .assert()
    .success()
    .stdout(predicate::str::contains(".hidden.png"));
}

#[test]
fn multiple_absolute_directory_patterns_each_match() {
  let first = TempDir::new().unwrap();
  let second = TempDir::new().unwrap();
  write(&first, "a.png");
  write(&second, "b.png");
  let cwd = TempDir::new().unwrap();
  imageoptim(&cwd)
    .args(["--dry-run"])
    .arg(first.path())
    .arg(second.path())
    .assert()
    .success()
    .stdout(predicate::str::contains("a.png"))
    .stdout(predicate::str::contains("b.png"));
}

#[test]
fn rejects_extended_glob_syntax_with_a_hint() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  for pattern in ["*.@(png|jpg)", "!(backup)*.png", "+(a|b).png", "?(x)y.png", "*(z).png"] {
    imageoptim(&dir)
      .args(["--dry-run", pattern])
      .assert()
      .code(1)
      .stderr(predicate::str::contains("Extended glob syntax"))
      .stderr(predicate::str::contains("{a,b}"));
  }
}

#[test]
fn rejects_numeric_brace_ranges_with_a_hint() {
  let dir = TempDir::new().unwrap();
  write(&dir, "pixel.png");
  imageoptim(&dir)
    .args(["--dry-run", "img{1..3}.png"])
    .assert()
    .code(1)
    .stderr(predicate::str::contains("{1,2,3}").or(predicate::str::contains("Brace ranges")));
}

#[test]
fn still_accepts_plain_brace_alternation() {
  let dir = TempDir::new().unwrap();
  write(&dir, "brace-a.png");
  write(&dir, "brace-b.png");
  imageoptim(&dir)
    .args(["--dry-run", "brace-{a,b}.png"])
    .assert()
    .success()
    .stdout(predicate::str::contains("brace-a.png"))
    .stdout(predicate::str::contains("brace-b.png"));
}

#[test]
fn matches_hidden_directories_which_a_pattern_explicitly_names() {
  let dir = TempDir::new().unwrap();
  write(&dir, "cache/.thumbs/small.png");
  write(&dir, "cache/visible.png");
  imageoptim(&dir)
    .args(["--dry-run", "*/.thumbs/*.png"])
    .assert()
    .success()
    .stdout(predicate::str::contains("cache/.thumbs/small.png"));
}

#[test]
fn wildcard_walks_still_skip_hidden_directories_by_default() {
  let dir = TempDir::new().unwrap();
  write(&dir, "cache/.thumbs/small.png");
  write(&dir, "cache/visible.png");
  imageoptim(&dir)
    .args(["--dry-run", "**/*.png"])
    .assert()
    .success()
    .stdout(predicate::str::contains("visible.png"))
    .stdout(predicate::str::contains(".thumbs").not());
}
