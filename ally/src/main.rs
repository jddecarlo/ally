use clap::{arg, command, Command};
use ally_lib::commands::*;

fn main() -> Result<(), ()> {
    let matches =
        command!()
            .subcommand(
                Command::new("FixPathSeparators")
                    .about("Fixes path separators in given string.")
                    .arg(arg!([PATH] "Path to fix the separators in.")
                        .required(true))
                    .alias("fps"))
            .get_matches();

    if let Some(matches) = matches.subcommand_matches("FixPathSeparators") {
        FixPathSeparatorsCommand::new(matches.value_of("PATH").unwrap().to_string()).execute()?
    }

    Ok(())
}
