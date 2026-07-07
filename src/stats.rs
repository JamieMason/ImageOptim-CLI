const UNITS: [&str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

#[derive(Debug, PartialEq)]
pub struct FileStat {
  pub path: String,
  pub before: u64,
  pub after: u64,
}

impl FileStat {
  pub fn new(path: impl Into<String>, before: u64, after: u64) -> Self {
    Self {
      path: path.into(),
      before,
      after,
    }
  }

  pub fn saving(&self) -> i64 {
    self.before as i64 - self.after as i64
  }

  pub fn percent_saving(&self) -> f64 {
    if self.before == 0 {
      return 0.0;
    }
    (self.saving() as f64 / self.before as f64) * 100.0
  }

  pub fn grew(&self) -> bool {
    self.saving() < 0
  }
}

#[derive(Debug, PartialEq)]
pub struct Stats {
  pub files: Vec<FileStat>,
  pub total: FileStat,
}

impl FromIterator<FileStat> for Stats {
  fn from_iter<I: IntoIterator<Item = FileStat>>(files: I) -> Self {
    let files: Vec<FileStat> = files.into_iter().collect();
    let total = FileStat::new(
      "TOTAL",
      files.iter().map(|file| file.before).sum(),
      files.iter().map(|file| file.after).sum(),
    );
    Self { files, total }
  }
}

/// Match the output of the pretty-bytes npm package with its space removed:
/// 1000-based units, 3 significant digits, trailing zeroes dropped
pub fn format_size(bytes: i64) -> String {
  let sign = if bytes < 0 { "-" } else { "" };
  let magnitude = bytes.unsigned_abs();
  if magnitude < 1000 {
    return format!("{sign}{magnitude}B");
  }
  let exponent = (magnitude.ilog10() / 3).min(UNITS.len() as u32 - 1);
  let scaled = magnitude as f64 / 1000f64.powi(exponent as i32);
  let scaled = to_precision_3(scaled);
  format!("{sign}{scaled}{}", UNITS[exponent as usize])
}

/// JavaScript's Number(value.toPrecision(3))
fn to_precision_3(value: f64) -> f64 {
  let digits_before_point = value.abs().log10().floor() as i32;
  let factor = 10f64.powi(2 - digits_before_point);
  (value * factor).round() / factor
}

#[cfg(test)]
#[path = "stats_test.rs"]
mod stats_test;
