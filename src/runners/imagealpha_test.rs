use {
  super::*,
  crate::test::outcome_with_status,
  std::sync::{Arc, Mutex},
};

type Calls = Arc<Mutex<Vec<Vec<String>>>>;

fn quality(min: u8, max: u8) -> Quality {
  Quality { min, max }
}

fn recording_runner(exit_status: i32, calls: &Calls) -> ImageAlpha {
  let calls = Arc::clone(calls);
  ImageAlpha {
    binary_exists: Box::new(|| true),
    run_pngquant: Box::new(move |args| {
      calls
        .lock()
        .unwrap()
        .push(args.iter().map(|arg| arg.to_string_lossy().into_owned()).collect());
      Ok(outcome_with_status(exit_status))
    }),
    max_shards: 1,
  }
}

/// A pngquant which fails the way it does on a file it cannot decode: the
/// offending file is named on stderr and the run exits READ_ERROR
fn unreadable_file_runner(stderr: &'static str) -> ImageAlpha {
  ImageAlpha {
    binary_exists: Box::new(|| true),
    run_pngquant: Box::new(move |_| {
      Ok(ExecOutcome {
        status: Some(READ_ERROR),
        stdout: String::new(),
        stderr: stderr.to_string(),
      })
    }),
    max_shards: 1,
  }
}

/// Split one shard's recorded args into its flags and its file paths
fn flags_and_files(args: &[String]) -> (Vec<String>, Vec<String>) {
  let separator = args.iter().position(|arg| arg == "--").unwrap();
  (args[..separator].to_vec(), args[separator + 1..].to_vec())
}

#[test]
fn passes_settings_and_png_paths_to_pngquant() {
  let calls: Calls = Arc::default();
  let runner = recording_runner(0, &calls);
  runner
    .run(
      &[PathBuf::from("a.png"), PathBuf::from("b.png")],
      &quality(65, 80),
      1,
      256,
      &Reporter::Human,
    )
    .unwrap();
  assert_eq!(
    calls.lock().unwrap().as_slice(),
    [vec![
      "--ext=.png".to_string(),
      "--force".to_string(),
      "--skip-if-larger".to_string(),
      "--speed=1".to_string(),
      "--quality=65-80".to_string(),
      "256".to_string(),
      "--".to_string(),
      "a.png".to_string(),
      "b.png".to_string(),
    ]]
  );
}

#[test]
fn shards_pngs_across_parallel_pngquant_invocations() {
  let calls: Calls = Arc::default();
  let runner = ImageAlpha {
    max_shards: 2,
    ..recording_runner(0, &calls)
  };
  let files: Vec<PathBuf> = ["a.png", "b.png", "c.png", "d.png"].iter().map(PathBuf::from).collect();
  runner.run(&files, &quality(65, 80), 1, 256, &Reporter::Human).unwrap();
  let calls = calls.lock().unwrap();
  assert_eq!(calls.len(), 2, "expected one pngquant invocation per shard: {calls:?}");
  let mut seen_files: Vec<String> = vec![];
  for shard_args in calls.iter() {
    let (flags, shard_files) = flags_and_files(shard_args);
    assert_eq!(
      flags,
      ["--ext=.png", "--force", "--skip-if-larger", "--speed=1", "--quality=65-80", "256"]
    );
    assert_eq!(shard_files.len(), 2, "files must be spread evenly: {calls:?}");
    seen_files.extend(shard_files);
  }
  seen_files.sort();
  assert_eq!(seen_files, ["a.png", "b.png", "c.png", "d.png"]);
}

#[test]
fn never_runs_more_shards_than_files() {
  let calls: Calls = Arc::default();
  let runner = ImageAlpha {
    max_shards: 8,
    ..recording_runner(0, &calls)
  };
  runner
    .run(&[PathBuf::from("a.png")], &quality(65, 80), 1, 256, &Reporter::Human)
    .unwrap();
  assert_eq!(calls.lock().unwrap().len(), 1);
}

#[test]
fn tolerates_skipped_files_in_one_shard_while_others_succeed() {
  let calls: Calls = Arc::default();
  let recorded = Arc::clone(&calls);
  let runner = ImageAlpha {
    max_shards: 2,
    binary_exists: Box::new(|| true),
    // The shard containing b.png exits 99 (quality unreachable), the other 0
    run_pngquant: Box::new(move |args| {
      recorded
        .lock()
        .unwrap()
        .push(args.iter().map(|arg| arg.to_string_lossy().into_owned()).collect());
      let status = if args.iter().any(|arg| arg == "b.png") { 99 } else { 0 };
      Ok(outcome_with_status(status))
    }),
  };
  let files: Vec<PathBuf> = ["a.png", "b.png", "c.png", "d.png"].iter().map(PathBuf::from).collect();
  assert!(runner.run(&files, &quality(65, 80), 1, 256, &Reporter::Human).is_ok());
  assert_eq!(calls.lock().unwrap().len(), 2);
}

#[test]
fn fails_when_any_shard_exits_with_a_hard_error() {
  let calls: Calls = Arc::default();
  let runner = ImageAlpha {
    max_shards: 2,
    ..recording_runner(17, &calls)
  };
  let files: Vec<PathBuf> = ["a.png", "b.png", "c.png", "d.png"].iter().map(PathBuf::from).collect();
  assert!(matches!(
    runner.run(&files, &quality(65, 80), 1, 256, &Reporter::Human),
    Err(ImageOptimError::AppFailed {
      app: "pngquant",
      status: 17,
      ..
    })
  ));
}

#[test]
fn does_not_run_pngquant_when_there_are_no_pngs() {
  let calls: Calls = Arc::default();
  let runner = recording_runner(0, &calls);
  runner.run(&[], &quality(65, 80), 1, 256, &Reporter::Human).unwrap();
  assert!(calls.lock().unwrap().is_empty());
}

#[test]
fn errors_when_imagealpha_is_not_installed() {
  let calls: Calls = Arc::default();
  let runner = ImageAlpha {
    binary_exists: Box::new(|| false),
    ..recording_runner(0, &calls)
  };
  assert!(matches!(
    runner.run(&[PathBuf::from("a.png")], &quality(65, 80), 1, 256, &Reporter::Human),
    Err(ImageOptimError::ImageAlphaNotInstalled)
  ));
}

#[test]
fn tolerates_pngquant_skipping_files() {
  for status in [98, 99] {
    let calls: Calls = Arc::default();
    let runner = recording_runner(status, &calls);
    assert!(
      runner
        .run(&[PathBuf::from("a.png")], &quality(65, 80), 1, 256, &Reporter::Human)
        .is_ok()
    );
  }
}

#[test]
fn fails_on_any_other_pngquant_exit_status() {
  let calls: Calls = Arc::default();
  let runner = recording_runner(17, &calls);
  assert!(matches!(
    runner.run(&[PathBuf::from("a.png")], &quality(65, 80), 1, 256, &Reporter::Human),
    Err(ImageOptimError::AppFailed {
      app: "pngquant",
      status: 17,
      ..
    })
  ));
}

/// Malformed images are the user's data to fix, so pngquant's READ_ERROR is
/// reported and skipped rather than failing the run, as issue #185 asked
#[test]
fn tolerates_pngs_which_pngquant_cannot_decode() {
  let runner = unreadable_file_runner("error:  'a.png' is not a PNG file (invalid PNG header)");
  assert!(
    runner
      .run(&[PathBuf::from("a.png")], &quality(65, 80), 1, 256, &Reporter::Human)
      .is_ok()
  );
}

#[test]
fn reports_every_file_pngquant_named_on_stderr() {
  assert_eq!(
    unreadable_reports("error:  'a.png' is not a PNG file\n\n  error:  cannot open b.png for reading\n"),
    ["error:  'a.png' is not a PNG file", "error:  cannot open b.png for reading"],
    "blank lines must not become skip reports"
  );
}

/// Some pngquant builds exit READ_ERROR without explaining themselves: the
/// user must still be told their files were left alone
#[test]
fn reports_a_fallback_when_pngquant_explains_nothing() {
  assert_eq!(
    unreadable_reports("   \n\n"),
    ["pngquant could not read one or more PNGs, they were left unchanged"]
  );
  let runner = unreadable_file_runner("");
  assert!(
    runner
      .run(&[PathBuf::from("a.png")], &quality(65, 80), 1, 256, &Reporter::Human)
      .is_ok()
  );
}
