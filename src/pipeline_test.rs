use {
  super::*,
  crate::{
    apps::App,
    reporter::Reporter,
    test::{TestFileSystem, ok_outcome},
  },
  std::{
    io,
    sync::{Arc, Mutex},
  },
};

type Events = Arc<Mutex<Vec<String>>>;

fn cli(args: &[&str]) -> Cli {
  Cli::parse_from_args(std::iter::once("imageoptim").chain(args.iter().copied())).unwrap()
}

fn fake_imagealpha(events: &Events) -> ImageAlpha {
  let events = Arc::clone(events);
  ImageAlpha {
    max_shards: 1,
    binary_exists: Box::new(|| true),
    run_pngquant: Box::new(move |args| {
      let files: Vec<String> = args
        .iter()
        .skip_while(|arg| *arg != "--")
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
      events.lock().unwrap().push(format!("imagealpha {}", files.join(" ")));
      Ok(ok_outcome())
    }),
  }
}

fn fake_imageoptim(events: &Events) -> ImageOptim {
  let events = Arc::clone(events);
  ImageOptim {
    binary_exists: Box::new(|| true),
    run_imageoptim: Box::new(move |files| {
      let files: Vec<String> = files.iter().map(|file| file.display().to_string()).collect();
      events.lock().unwrap().push(format!("imageoptim {}", files.join(" ")));
      Ok(ok_outcome())
    }),
  }
}

fn fake_jpegmini(events: &Events) -> JpegMini {
  let events = Arc::clone(events);
  let automation_events = Arc::clone(&events);
  JpegMini {
    is_installed: Box::new(|app: &App| Ok(app.bundle_id == "com.icvt.JPEGmini-Pro")),
    supports_assistive_devices: Box::new(|| Ok(true)),
    automate_folder: Box::new(move |_dir, _app| {
      automation_events.lock().unwrap().push("jpegmini".to_string());
      Ok(())
    }),
    quit: Box::new(move |app| {
      events.lock().unwrap().push(format!("quit {}", app.name));
      Ok(())
    }),
  }
}

fn events_after_run(cli: &Cli, files: &[&str], runners: &Runners, events: &Events) -> Vec<String> {
  let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
  run(cli, &files, &TestFileSystem::default(), runners, &Reporter::Human).unwrap();
  let events = events.lock().unwrap();
  events.clone()
}

#[test]
fn runs_imagealpha_and_jpegmini_before_imageoptim() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: Some(fake_imagealpha(&events)),
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: Some(fake_jpegmini(&events)),
  };
  let events = events_after_run(&cli(&["-a", "-j"]), &["a.png", "b.jpg"], &runners, &events);
  let imageoptim_position = events.iter().position(|event| event.starts_with("imageoptim")).unwrap();
  assert!(events.iter().any(|event| event.starts_with("imagealpha")), "{events:?}");
  assert!(events.contains(&"jpegmini".to_string()), "{events:?}");
  assert_eq!(imageoptim_position, events.len() - 1, "ImageOptim must run last: {events:?}");
}

#[test]
fn sends_only_pngs_to_imagealpha_and_only_jpegs_to_jpegmini() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: Some(fake_imagealpha(&events)),
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: None,
  };
  let events = events_after_run(&cli(&["-a"]), &["a.png", "b.jpg", "c.gif"], &runners, &events);
  assert!(events.contains(&"imagealpha a.png".to_string()), "{events:?}");
  assert!(events.contains(&"imageoptim a.png b.jpg c.gif".to_string()), "{events:?}");
}

#[test]
fn skips_disabled_apps() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: None,
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: None,
  };
  let events = events_after_run(&cli(&[]), &["a.png"], &runners, &events);
  assert_eq!(events, ["imageoptim a.png"]);
}

#[test]
fn processes_files_in_batches() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: None,
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: None,
  };
  let events = events_after_run(&cli(&["--batch-size", "2"]), &["a.png", "b.png", "c.png"], &runners, &events);
  assert_eq!(events, ["imageoptim a.png b.png", "imageoptim c.png"]);
}

#[test]
fn stops_at_the_first_failing_batch() {
  let calls = Arc::new(Mutex::new(0));
  let recorded = Arc::clone(&calls);
  let runners = Runners {
    imagealpha: None,
    imageoptim: Some(ImageOptim {
      binary_exists: Box::new(|| true),
      run_imageoptim: Box::new(move |_| {
        *recorded.lock().unwrap() += 1;
        Err(io::Error::other("boom"))
      }),
    }),
    jpegmini: None,
  };
  let files = [PathBuf::from("a.png"), PathBuf::from("b.png")];
  let cli = cli(&["--batch-size", "1"]);
  assert!(run(&cli, &files, &TestFileSystem::default(), &runners, &Reporter::Human).is_err());
  assert_eq!(*calls.lock().unwrap(), 1);
}

#[test]
fn skips_files_which_vanished_before_processing() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: None,
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: None,
  };
  let fs = TestFileSystem {
    missing_paths: vec![PathBuf::from("ghost.png")],
    ..TestFileSystem::default()
  };
  let files = [PathBuf::from("a.png"), PathBuf::from("ghost.png")];
  run(&cli(&[]), &files, &fs, &runners, &Reporter::Human).unwrap();
  assert_eq!(*events.lock().unwrap(), ["imageoptim a.png"]);
}

#[test]
fn continues_when_a_file_vanishes_during_processing() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: None,
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: None,
  };
  let fs = TestFileSystem {
    vanishing_paths: vec![PathBuf::from("b.png")],
    ..TestFileSystem::default()
  };
  let files = [PathBuf::from("a.png"), PathBuf::from("b.png")];
  run(&cli(&[]), &files, &fs, &runners, &Reporter::Human).unwrap();
  assert_eq!(*events.lock().unwrap(), ["imageoptim a.png b.png"]);
}

#[test]
fn skips_a_batch_whose_files_all_vanished() {
  let events: Events = Arc::default();
  let runners = Runners {
    imagealpha: None,
    imageoptim: Some(fake_imageoptim(&events)),
    jpegmini: None,
  };
  let fs = TestFileSystem {
    missing_paths: vec![PathBuf::from("ghost.png")],
    ..TestFileSystem::default()
  };
  run(&cli(&[]), &[PathBuf::from("ghost.png")], &fs, &runners, &Reporter::Human).unwrap();
  assert!(events.lock().unwrap().is_empty(), "no app should run over an empty batch");
}
