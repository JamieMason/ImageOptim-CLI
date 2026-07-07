use {
  super::*,
  crate::test::{ok_outcome, outcome_with_status},
  std::sync::{Arc, Mutex},
};

#[test]
fn passes_every_file_to_the_imageoptim_binary() {
  let calls: Arc<Mutex<Vec<Vec<PathBuf>>>> = Arc::default();
  let recorded = Arc::clone(&calls);
  let runner = ImageOptim {
    binary_exists: Box::new(|| true),
    run_imageoptim: Box::new(move |files| {
      recorded.lock().unwrap().push(files.to_vec());
      Ok(ok_outcome())
    }),
  };
  let files = [PathBuf::from("a.png"), PathBuf::from("b.jpg")];
  runner.run(&files).unwrap();
  assert_eq!(calls.lock().unwrap().as_slice(), [files.to_vec()]);
}

#[test]
fn does_not_run_imageoptim_with_no_files() {
  let calls: Arc<Mutex<Vec<Vec<PathBuf>>>> = Arc::default();
  let recorded = Arc::clone(&calls);
  let runner = ImageOptim {
    binary_exists: Box::new(|| true),
    run_imageoptim: Box::new(move |files| {
      recorded.lock().unwrap().push(files.to_vec());
      Ok(ok_outcome())
    }),
  };
  runner.run(&[]).unwrap();
  assert!(calls.lock().unwrap().is_empty());
}

#[test]
fn errors_when_imageoptim_is_not_installed() {
  let runner = ImageOptim {
    binary_exists: Box::new(|| false),
    run_imageoptim: Box::new(|_| Ok(ok_outcome())),
  };
  assert!(matches!(
    runner.run(&[PathBuf::from("a.png")]),
    Err(ImageOptimError::ImageOptimNotInstalled)
  ));
}

#[test]
fn fails_when_imageoptim_exits_with_an_error() {
  let runner = ImageOptim {
    binary_exists: Box::new(|| true),
    run_imageoptim: Box::new(|_| Ok(outcome_with_status(70))),
  };
  assert!(matches!(
    runner.run(&[PathBuf::from("a.png")]),
    Err(ImageOptimError::AppFailed {
      app: "ImageOptim",
      status: 70,
      ..
    })
  ));
}

#[test]
fn prefixes_dash_leading_paths_so_they_are_not_read_as_flags() {
  let calls: Arc<Mutex<Vec<Vec<PathBuf>>>> = Arc::default();
  let recorded = Arc::clone(&calls);
  let runner = ImageOptim {
    binary_exists: Box::new(|| true),
    run_imageoptim: Box::new(move |files| {
      recorded.lock().unwrap().push(files.to_vec());
      Ok(ok_outcome())
    }),
  };
  runner.run(&[PathBuf::from("-photo.png"), PathBuf::from("safe.png")]).unwrap();
  assert_eq!(
    calls.lock().unwrap().as_slice(),
    [vec![PathBuf::from("./-photo.png"), PathBuf::from("safe.png")]]
  );
}
