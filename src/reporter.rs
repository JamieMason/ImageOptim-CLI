use {
  crate::{logger, report, stats::Stats},
  serde_json::json,
  std::path::Path,
};

/// Where output goes and what it looks like: human-readable text, or one JSON
/// object per line for machines. JSON goes to stdout; human status lines go to
/// stderr so that stdout stays parseable
pub enum Reporter {
  Human,
  Json,
}

impl Reporter {
  pub fn new(json: bool) -> Self {
    if json { Self::Json } else { Self::Human }
  }

  /// A run is starting over this many files
  pub fn on_start(&self, file_count: usize) {
    if let Self::Json = self {
      println!(
        "{}",
        json!({ "type": "start", "version": env!("CARGO_PKG_VERSION"), "files": file_count })
      );
    }
  }

  /// A file matched the patterns during --dry-run
  pub fn on_match(&self, path: &Path) {
    match self {
      Self::Human => println!("{}", path.display()),
      Self::Json => println!("{}", json!({ "type": "match", "path": path.display().to_string() })),
    }
  }

  /// An app is about to process a batch
  pub fn on_app_start(&self, app_name: &str) {
    match self {
      Self::Human => eprintln!("{}", logger::info(&format!("Running {app_name}..."))),
      Self::Json => println!("{}", json!({ "type": "app_start", "app": app_name })),
    }
  }

  /// A batch finished and its files were measured
  pub fn on_batch_stats(&self, stats: &Stats) {
    match self {
      Self::Human => {
        for line in report::render(stats) {
          println!("{line}");
        }
      }
      Self::Json => {
        for file in &stats.files {
          println!(
            "{}",
            json!({
              "type": "file",
              "path": file.path,
              "before": file.before,
              "after": file.after,
              "saving": file.saving(),
              "percent": file.percent_saving()
            })
          );
        }
        println!(
          "{}",
          json!({
            "type": "total",
            "before": stats.total.before,
            "after": stats.total.after,
            "saving": stats.total.saving(),
            "percent": stats.total.percent_saving()
          })
        );
      }
    }
  }

  /// Everything completed successfully
  pub fn on_finished(&self) {
    match self {
      Self::Human => println!("{}", logger::complete("Finished")),
      Self::Json => println!("{}", json!({ "type": "done" })),
    }
  }

  /// The run failed: the human-readable version goes to stderr in main
  pub fn on_error(&self, message: &str) {
    if let Self::Json = self {
      println!("{}", json!({ "type": "error", "message": message }));
    }
  }
}
