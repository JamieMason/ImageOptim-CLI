use {
  super::*,
  crate::{apps::JPEGMINI_VARIANTS, reporter::Reporter, test::TestFileSystem},
  std::sync::{Arc, Mutex},
};

type Events = Arc<Mutex<Vec<String>>>;

fn jpegs() -> Vec<PathBuf> {
  vec![PathBuf::from("a.jpg")]
}

fn installed_only(bundle_id: &'static str) -> IsInstalled {
  Box::new(move |app| Ok(app.bundle_id == bundle_id))
}

fn runner(is_installed: IsInstalled, events: &Events) -> JpegMini {
  let automation_events = Arc::clone(events);
  let quit_events = Arc::clone(events);
  JpegMini {
    is_installed,
    supports_assistive_devices: Box::new(|| Ok(true)),
    automate_folder: Box::new(move |dir, app| {
      automation_events
        .lock()
        .unwrap()
        .push(format!("automate {} in {}", app.name, dir.display()));
      Ok(())
    }),
    quit: Box::new(move |app| {
      quit_events.lock().unwrap().push(format!("quit {}", app.name));
      Ok(())
    }),
  }
}

#[test]
fn checks_each_jpegmini_variant_in_priority_order() {
  let checked: Arc<Mutex<Vec<&str>>> = Arc::default();
  let recorded = Arc::clone(&checked);
  let events: Events = Arc::default();
  let runner = JpegMini {
    is_installed: Box::new(move |app| {
      recorded.lock().unwrap().push(app.bundle_id);
      Ok(false)
    }),
    ..runner(installed_only(""), &events)
  };
  let _ = runner.run(&jpegs(), &TestFileSystem::default(), &Reporter::Human);
  assert_eq!(
    checked.lock().unwrap().as_slice(),
    [
      "com.icvt.JPEGmini-Pro",
      "com.icvt.JPEGmini-Pro-retail",
      "com.icvt.JPEGmini",
      "com.icvt.JPEGmini-retail",
      "com.icvt.JPEGminiLite",
      "com.icvt.JPEGminiLite-retail",
      "com.beamr.jpegminipro.app",
    ]
  );
}

#[test]
fn stops_checking_once_a_variant_is_found() {
  let events: Events = Arc::default();
  let runner = runner(installed_only("com.icvt.JPEGmini"), &events);
  let fs = TestFileSystem::default();
  runner.run(&jpegs(), &fs, &Reporter::Human).unwrap();
  let events = events.lock().unwrap();
  let base = symlink_dir_base().display().to_string();
  assert!(events[0].starts_with(&format!("automate JPEGmini in {base}")), "{events:?}");
  assert_eq!(events[1], "quit JPEGmini");
}

#[test]
fn errors_when_no_variant_is_installed() {
  let events: Events = Arc::default();
  let runner = runner(installed_only("nothing"), &events);
  assert!(matches!(
    runner.run(&jpegs(), &TestFileSystem::default(), &Reporter::Human),
    Err(ImageOptimError::JpegMiniNotInstalled)
  ));
  assert!(events.lock().unwrap().is_empty());
}

#[test]
fn errors_when_assistive_devices_are_unsupported() {
  let events: Events = Arc::default();
  let runner = JpegMini {
    supports_assistive_devices: Box::new(|| Ok(false)),
    ..runner(installed_only("com.icvt.JPEGmini-Pro"), &events)
  };
  assert!(matches!(
    runner.run(&jpegs(), &TestFileSystem::default(), &Reporter::Human),
    Err(ImageOptimError::AssistiveDevicesUnsupported)
  ));
  assert!(events.lock().unwrap().is_empty());
}

#[test]
fn does_nothing_when_there_are_no_jpegs() {
  let checked: Arc<Mutex<Vec<&str>>> = Arc::default();
  let recorded = Arc::clone(&checked);
  let events: Events = Arc::default();
  let runner = JpegMini {
    is_installed: Box::new(move |app| {
      recorded.lock().unwrap().push(app.bundle_id);
      Ok(true)
    }),
    ..runner(installed_only(""), &events)
  };
  runner.run(&[], &TestFileSystem::default(), &Reporter::Human).unwrap();
  assert!(checked.lock().unwrap().is_empty());
  assert!(events.lock().unwrap().is_empty());
}

#[test]
fn fills_a_symlink_dir_with_the_jpegs_then_removes_it() {
  let events: Events = Arc::default();
  let runner = runner(installed_only("com.icvt.JPEGmini-Pro"), &events);
  let fs = TestFileSystem::default();
  runner.run(&jpegs(), &fs, &Reporter::Human).unwrap();
  let created = fs.created_symlink_dirs.lock().unwrap();
  let removed = fs.removed_dirs.lock().unwrap();
  assert_eq!(created.len(), 1);
  let (dir, files) = &created[0];
  assert!(dir.starts_with(symlink_dir_base()), "{}", dir.display());
  assert_eq!(files, &jpegs());
  assert_eq!(removed.as_slice(), std::slice::from_ref(dir));
}

#[test]
fn removes_the_symlink_dir_when_creating_symlinks_fails() {
  let events: Events = Arc::default();
  let runner = runner(installed_only("com.icvt.JPEGmini-Pro"), &events);
  let fs = TestFileSystem {
    fail_create_symlink_dir: true,
    ..TestFileSystem::default()
  };
  assert!(runner.run(&jpegs(), &fs, &Reporter::Human).is_err());
  let created = fs.created_symlink_dirs.lock().unwrap();
  let removed = fs.removed_dirs.lock().unwrap();
  assert_eq!(created.len(), 1);
  assert_eq!(removed.as_slice(), std::slice::from_ref(&created[0].0));
  assert!(events.lock().unwrap().is_empty(), "automation must not run after a failed setup");
}

#[test]
fn removes_the_symlink_dir_when_automation_fails() {
  let events: Events = Arc::default();
  let runner = JpegMini {
    automate_folder: Box::new(|_, _| Err(io::Error::other("JPEGmini crashed"))),
    ..runner(installed_only("com.icvt.JPEGmini-Pro"), &events)
  };
  let fs = TestFileSystem::default();
  assert!(runner.run(&jpegs(), &fs, &Reporter::Human).is_err());
  let created = fs.created_symlink_dirs.lock().unwrap();
  let removed = fs.removed_dirs.lock().unwrap();
  assert_eq!(created.len(), 1);
  assert_eq!(removed.as_slice(), [created[0].0.clone()]);
}

#[test]
fn every_variant_maps_to_its_marketing_name() {
  let names: Vec<&str> = JPEGMINI_VARIANTS.iter().map(|app| app.name).collect();
  assert_eq!(
    names,
    [
      "JPEGmini Pro",
      "JPEGmini Pro",
      "JPEGmini",
      "JPEGmini",
      "JPEGmini Lite",
      "JPEGmini Lite",
      "JPEGmini Pro"
    ]
  );
}
