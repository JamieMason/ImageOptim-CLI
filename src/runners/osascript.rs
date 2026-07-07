use {
  crate::{apps, runners::exec},
  std::{ffi::OsStr, io},
};

pub const IS_INSTALLED: &str = include_str!("osascript/is-installed.applescript");
pub const QUIT_APP: &str = include_str!("osascript/quit-app.applescript");
pub const RUN_JPEGMINI: &str = include_str!("osascript/run-jpegmini.applescript");
pub const SUPPORTS_ASSISTIVE_DEVICES: &str = include_str!("osascript/supports-assistive-devices.applescript");

/// Run an embedded AppleScript with osascript -e, returning its trimmed output
pub fn run_script(script: &str, args: &[&str]) -> io::Result<String> {
  let program = apps::osascript_path();
  let mut exec_args: Vec<&OsStr> = vec![OsStr::new("-e"), OsStr::new(script)];
  exec_args.extend(args.iter().map(OsStr::new));
  let outcome = exec(program.as_os_str(), &exec_args)?;
  match outcome.status {
    Some(0) => Ok(outcome.stdout.trim().to_string()),
    status => Err(io::Error::other(format!(
      "osascript exited with status {}: {}",
      status.map_or("none".to_string(), |code| code.to_string()),
      outcome.stderr.trim()
    ))),
  }
}
