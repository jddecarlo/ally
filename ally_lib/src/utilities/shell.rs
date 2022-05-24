use std::io::{self, Write};
use std::process::{Command, Output, Stdio};
use crate::{AllyError, AllyResult};

pub(crate) fn execute_shell_command(command: &str, args: &[&str], stdin_text: Option<&str>) -> AllyResult<Output, io::Error> {
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
            stdin.write_all(stdin_text.as_bytes())?;
        }
    }

    Ok(child.wait_with_output()?)
}
