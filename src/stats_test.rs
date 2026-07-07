use super::*;

#[test]
fn formats_sizes_like_the_pretty_bytes_npm_package() {
  assert_eq!(format_size(0), "0B");
  assert_eq!(format_size(1), "1B");
  assert_eq!(format_size(999), "999B");
  assert_eq!(format_size(1000), "1kB");
  assert_eq!(format_size(1337), "1.34kB");
  assert_eq!(format_size(1500), "1.5kB");
  assert_eq!(format_size(99999), "100kB");
  assert_eq!(format_size(999949), "1000kB");
  assert_eq!(format_size(1000000), "1MB");
  assert_eq!(format_size(1550000), "1.55MB");
  assert_eq!(format_size(1234567890), "1.23GB");
}

#[test]
fn formats_negative_sizes_for_files_which_grew() {
  assert_eq!(format_size(-5), "-5B");
  assert_eq!(format_size(-1337), "-1.34kB");
}

#[test]
fn calculates_savings_per_file() {
  let stat = FileStat::new("pixel.png", 100, 90);
  assert_eq!(stat.saving(), 10);
  assert_eq!(stat.percent_saving(), 10.0);
  assert!(!stat.grew());
}

#[test]
fn calculates_negative_savings_for_files_which_grew() {
  let stat = FileStat::new("pixel.png", 100, 150);
  assert_eq!(stat.saving(), -50);
  assert_eq!(stat.percent_saving(), -50.0);
  assert!(stat.grew());
}

#[test]
fn reports_zero_percent_when_nothing_was_saved() {
  let stat = FileStat::new("pixel.png", 0, 0);
  assert_eq!(stat.percent_saving(), 0.0);
}

#[test]
fn reports_zero_percent_when_an_empty_file_grew() {
  let stat = FileStat::new("empty.png", 0, 5);
  assert!(stat.percent_saving().is_finite());
  assert_eq!(stat.percent_saving(), 0.0);
}

#[test]
fn totals_savings_across_all_files() {
  let stats: Stats = [FileStat::new("a.png", 100, 50), FileStat::new("b.png", 200, 100)]
    .into_iter()
    .collect();
  assert_eq!(stats.total, FileStat::new("TOTAL", 300, 150));
  assert_eq!(stats.total.saving(), 150);
  assert_eq!(stats.total.percent_saving(), 50.0);
}
