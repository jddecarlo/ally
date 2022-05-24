use std::error::Error;
use std::io;
use text_io::read;
use crate::AllyResult;
use crate::utilities::git;

pub trait Executable<T, E: Error> {
    fn execute(&self) -> AllyResult<T, E>;
}

pub struct FixPathSeparatorsCommand {
    pub path: Option<String>,
}

impl FixPathSeparatorsCommand {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl Executable<(), io::Error> for FixPathSeparatorsCommand {
    fn execute(&self) -> AllyResult<(), io::Error> {
        let input = match &self.path {
            Some(ref path) => path.to_owned(),
            None => read!("{}\0"),
        };

        let result;
        if cfg!(windows) {
            result = input.replace("/", "\\");
        } else {
            result = input.replace("\\", "/");
        }

        println!("{result}");
        Ok(())
    }
}

pub struct GitIncomingCommand { }

impl GitIncomingCommand {
    pub fn new() -> Self {
        Self { }
    }
}

impl Executable<(), io::Error> for GitIncomingCommand {
    fn execute(&self) -> AllyResult<(), io::Error> {
        git::print_incoming_commits()?;
        Ok(())
    }
}

pub struct GitOutgoingCommand { }

impl GitOutgoingCommand {
    pub fn new() -> Self {
        Self { }
    }
}

impl Executable<(), io::Error> for GitOutgoingCommand {
    fn execute(&self) -> AllyResult<(), io::Error> {
        git::print_outgoing_commits()?;
        Ok(())
    }
}

pub struct EnvironmentCommand { }

impl EnvironmentCommand {
    pub fn new() -> Self { Self { }}
}

impl Executable<(), io:Error> for EnvironmentCommand {
    fn execute(&self) -> AllyResult<(), io::Error> {
        todo!();
        Ok(())
    }
}