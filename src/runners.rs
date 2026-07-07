pub mod imagealpha;
pub mod imageoptim;
pub mod jpegmini;
pub mod osascript;

use {
  crate::logger,
  std::{ffi::OsStr, io, process::Command},
};

/// Check whether an app's binary is installed
pub type BinaryExists = Box<dyn Fn() -> bool + Sync>;

/// The outcome of running an external program: its exit status and captured
/// output. Capturing keeps other programs' stdout out of this CLI's report
pub struct ExecOutcome {
  pub status: Option<i32>,
  pub stdout: String,
  pub stderr: String,
}

pub fn exec(program: &OsStr, args: &[&OsStr]) -> io::Result<ExecOutcome> {
  logger::debug(&format!(
    "exec {} {}",
    program.display(),
    args.iter().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>().join(" ")
  ));
  let output = Command::new(program).args(args).output()?;
  Ok(ExecOutcome {
    status: output.status.code(),
    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
  })
}
