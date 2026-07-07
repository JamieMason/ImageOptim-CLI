use {super::*, std::fs, tempfile::TempDir};

#[test]
fn reads_file_sizes() {
  let dir = TempDir::new().unwrap();
  let path = dir.path().join("pixel.png");
  fs::write(&path, "1234").unwrap();
  assert_eq!(RealFileSystem.file_size(&path).unwrap(), 4);
}

#[test]
fn keeps_relative_paths_relative_inside_the_symlink_dir() {
  assert_eq!(as_relative(Path::new("photos/a.jpg")), PathBuf::from("photos/a.jpg"));
}

#[test]
fn strips_the_leading_slash_from_absolute_paths() {
  assert_eq!(
    as_relative(Path::new("/Users/jamie/photos/a.jpg")),
    PathBuf::from("Users/jamie/photos/a.jpg")
  );
}

#[test]
fn symlinks_resolve_to_the_original_files() {
  let source_dir = TempDir::new().unwrap();
  let symlink_dir = TempDir::new().unwrap();
  let symlink_dir = symlink_dir.path().join("links");
  fs::create_dir_all(source_dir.path().join("photos")).unwrap();
  let source = source_dir.path().join("photos/a.jpg");
  fs::write(&source, "jpg").unwrap();
  RealFileSystem
    .create_symlink_dir(&symlink_dir, std::slice::from_ref(&source))
    .unwrap();
  let link: PathBuf = [symlink_dir.as_path(), &as_relative(&source.canonicalize().unwrap())]
    .iter()
    .collect();
  assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
  assert_eq!(fs::read_to_string(&link).unwrap(), "jpg");
}

#[test]
fn parent_dir_components_cannot_escape_the_symlink_dir() {
  let source_dir = TempDir::new().unwrap();
  let symlink_dir = TempDir::new().unwrap();
  let symlink_dir = symlink_dir.path().join("links");
  fs::create_dir_all(source_dir.path().join("photos")).unwrap();
  let source = source_dir.path().join("photos/a.jpg");
  fs::write(&source, "jpg").unwrap();
  let sneaky = source_dir.path().join("photos/../photos/a.jpg");
  RealFileSystem.create_symlink_dir(&symlink_dir, &[sneaky]).unwrap();
  let link: PathBuf = [symlink_dir.as_path(), &as_relative(&source.canonicalize().unwrap())]
    .iter()
    .collect();
  assert!(
    link.symlink_metadata().unwrap().file_type().is_symlink(),
    "link must be inside the symlink dir"
  );
}

#[test]
fn different_spellings_of_the_same_file_share_one_symlink() {
  let source_dir = TempDir::new().unwrap();
  let symlink_dir = TempDir::new().unwrap();
  let symlink_dir = symlink_dir.path().join("links");
  fs::create_dir_all(source_dir.path().join("photos")).unwrap();
  let source = source_dir.path().join("photos/a.jpg");
  fs::write(&source, "jpg").unwrap();
  let sneaky = source_dir.path().join("photos/../photos/a.jpg");
  RealFileSystem.create_symlink_dir(&symlink_dir, &[source.clone(), sneaky]).unwrap();
  let link: PathBuf = [symlink_dir.as_path(), &as_relative(&source.canonicalize().unwrap())]
    .iter()
    .collect();
  assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
}

#[test]
fn removes_directories_recursively_and_tolerates_absence() {
  let dir = TempDir::new().unwrap();
  let target = dir.path().join("a/b/c");
  fs::create_dir_all(&target).unwrap();
  RealFileSystem.remove_dir(&dir.path().join("a")).unwrap();
  assert!(!dir.path().join("a").exists());
  RealFileSystem.remove_dir(&dir.path().join("missing")).unwrap();
}

#[test]
fn skips_symlinks_for_files_which_no_longer_exist() {
  let source_dir = TempDir::new().unwrap();
  let symlink_dir = TempDir::new().unwrap();
  let symlink_dir = symlink_dir.path().join("links");
  let real = source_dir.path().join("real.jpg");
  fs::write(&real, "jpg").unwrap();
  let ghost = source_dir.path().join("ghost.jpg");
  RealFileSystem.create_symlink_dir(&symlink_dir, &[ghost, real.clone()]).unwrap();
  let link: PathBuf = [symlink_dir.as_path(), &as_relative(&real.canonicalize().unwrap())]
    .iter()
    .collect();
  assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
}
