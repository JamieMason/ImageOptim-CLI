use {super::*, std::path::PathBuf};

#[test]
fn matches_lowercase_extensions() {
  assert!(is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("a.png")));
  assert!(is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("a.jpg")));
}

#[test]
fn matches_extensions_case_insensitively() {
  assert!(is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("a.PNG")));
  assert!(is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("a.Png")));
}

#[test]
fn rejects_unsupported_extensions() {
  assert!(!is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("a.txt")));
  assert!(!is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("png")));
  assert!(!is_supported_extension(&SUPPORTED_EXTENSIONS, &PathBuf::from("a")));
}

#[test]
fn jpegmini_only_supports_jpegs() {
  assert!(is_supported_extension(&JPEGMINI_EXTENSIONS, &PathBuf::from("a.jpg")));
  assert!(is_supported_extension(&JPEGMINI_EXTENSIONS, &PathBuf::from("a.jpeg")));
  assert!(!is_supported_extension(&JPEGMINI_EXTENSIONS, &PathBuf::from("a.png")));
}

#[test]
fn imagealpha_only_supports_pngs() {
  assert!(is_supported_extension(&IMAGEALPHA_EXTENSIONS, &PathBuf::from("a.png")));
  assert!(!is_supported_extension(&IMAGEALPHA_EXTENSIONS, &PathBuf::from("a.jpg")));
}
