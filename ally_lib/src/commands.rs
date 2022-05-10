use text_io::read;
use crate::utilities::*;
use crate::utilities::git::*;

pub trait Executable<R, E> {
    fn execute(&self) -> Result<R, E>;
}

pub struct FixPathSeparatorsCommand {
    pub path: Option<String>,
}

impl FixPathSeparatorsCommand {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl Executable<(), ()> for FixPathSeparatorsCommand {
    fn execute(&self) -> Result<(), ()> {
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
        Self {}
    }
}

impl Executable<(), ()> for GitIncomingCommand {
    fn execute(&self) -> Result<(), ()> {


        Ok(())
    }
}