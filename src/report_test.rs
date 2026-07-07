use super::*;

fn without_color() {
  colored::control::set_override(false);
}

#[test]
fn renders_one_row_per_file_plus_a_total() {
  without_color();
  let stats: Stats = [FileStat::new("a.png", 1000, 500), FileStat::new("b.png", 2000, 1000)]
    .into_iter()
    .collect();
  assert_eq!(
    render(&stats),
    vec![
      "✓ a.png was: 1kB now: 500B saving: 500B (50.00%)",
      "✓ b.png was: 2kB now: 1kB saving: 1kB (50.00%)",
      "✓ TOTAL was: 3kB now: 1.5kB saving: 1.5kB (50.00%)",
    ]
  );
}

#[test]
fn flags_files_which_grew_with_a_warning_icon() {
  without_color();
  let stats: Stats = [FileStat::new("grew.png", 100, 150), FileStat::new("shrank.png", 1000, 400)]
    .into_iter()
    .collect();
  let lines = render(&stats);
  assert_eq!(lines[0], "! grew.png was: 100B now: 150B saving: -50B (-50.00%)");
  assert_eq!(lines[1], "✓ shrank.png was: 1kB now: 400B saving: 600B (60.00%)");
  assert_eq!(lines[2], "✓ TOTAL was: 1.1kB now: 550B saving: 550B (50.00%)");
}

#[test]
fn warns_when_there_are_no_size_savings() {
  without_color();
  let stats: Stats = [FileStat::new("a.png", 100, 100)].into_iter().collect();
  assert_eq!(render(&stats), vec!["! No size savings"]);
}
