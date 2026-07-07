use {super::*, tempfile::TempDir};

#[test]
fn removes_registered_dirs_and_forgets_deregistered_ones() {
  let base = TempDir::new().unwrap();
  let doomed = base.path().join("doomed");
  let spared = base.path().join("spared");
  std::fs::create_dir_all(&doomed).unwrap();
  std::fs::create_dir_all(&spared).unwrap();
  register(&doomed);
  register(&spared);
  deregister(&spared);
  remove_registered();
  assert!(!doomed.exists());
  assert!(spared.exists());
}
