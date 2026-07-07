use {
  crate::{
    logger,
    stats::{FileStat, Stats, format_size},
  },
  colored::Colorize,
};

/// One line per file plus a TOTAL line, or a warning when nothing was saved.
/// Matches the report of the TypeScript CLI, with one addition: files which
/// grew are flagged with a yellow ! instead of a green ✓
pub fn render(stats: &Stats) -> Vec<String> {
  if stats.total.saving() == 0 {
    return vec![logger::warning("No size savings")];
  }
  stats.files.iter().chain([&stats.total]).map(render_row).collect()
}

fn render_row(stat: &FileStat) -> String {
  let icon = if stat.grew() { "!".yellow() } else { "✓".green() };
  format!(
    "{icon} {} was: {} now: {} saving: {} ({})",
    stat.path.underline(),
    format_size(stat.before as i64).red(),
    format_size(stat.after as i64).green(),
    format_size(stat.saving()).green(),
    format!("{:.2}%", stat.percent_saving()).green()
  )
}

#[cfg(test)]
#[path = "report_test.rs"]
mod report_test;
