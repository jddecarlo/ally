use clap::{arg, command, Command};
use ally_lib::commands::*;

fn main() -> Result<(), ()> {
    let matches =
        command!()
            .subcommand(
                Command::new("FixPathSeparators")
                    .about("Fixes path separators in given string.")
                    .arg(arg!([PATH] "Path to fix the separators in.")
                        .required(false))
                    .alias("fps"))
            .get_matches();

    if let Some(matches) = matches.subcommand_matches("FixPathSeparators") {
        let path = 
            if matches.is_present("PATH") {
                Some(matches.value_of("PATH").unwrap().to_string())
            } else {
                None
            };
        FixPathSeparatorsCommand::new(path).execute()?
    }

    Ok(())
}
