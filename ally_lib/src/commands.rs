pub trait Executable<R, E> {
    fn execute(&self) -> Result<R, E>;
}

pub struct FixPathSeparatorsCommand {
    pub path: String,
}

impl FixPathSeparatorsCommand {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Executable<(), ()> for FixPathSeparatorsCommand {
    fn execute(&self) -> Result<(), ()> {
        let result;
        if cfg!(windows) {
            result = self.path.replace("/", "\\");
        } else {
            result = self.path.replace("\\", "/");
        }

        println!("{result}");
        Ok(())
    }
}
