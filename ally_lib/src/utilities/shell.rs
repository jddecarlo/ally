use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use crate::AllyResult;

pub(crate) fn execute_shell_command<T: AsRef<OsStr>>(command: T, args: &[T], stdin_text: Option<T>) -> AllyResult<Output> {
    let mut command = Command::new(command);
    let mut child = match stdin_text {
        Some(_) => {
            command
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .args(args)
                .spawn()?
        }
        None => {
            command
                .stdout(Stdio::piped())
                .args(args)
                .spawn()?
        }
    };
    
    if let Some(stdin_text) = stdin_text {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(stdin_text.as_ref().to_string_lossy().as_bytes())?;
        }
    }

    Ok(child.wait_with_output()?)
}
