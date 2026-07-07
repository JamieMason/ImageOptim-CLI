use {
  colored::Colorize,
  std::sync::atomic::{AtomicBool, Ordering},
};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(enabled: bool) {
  VERBOSE.store(enabled, Ordering::Relaxed);
}

/// Print a verbose line to stderr, when --verbose is set
pub fn debug(value: &str) {
  if VERBOSE.load(Ordering::Relaxed) {
    eprintln!("{}", verbose(value));
  }
}

pub fn complete(value: &str) -> String {
  format!("{} {value}", "✓".green())
}

pub fn info(value: &str) -> String {
  format!("{} {value}", "i".blue())
}

pub fn warning(value: &str) -> String {
  format!("{} {value}", "!".yellow())
}

pub fn error(value: &str) -> String {
  format!("{} {value}", "!".red())
}

pub fn verbose(value: &str) -> String {
  format!("{} {value}", "?".dimmed())
}
