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
            .subcommand(
                Command::new("GitIncoming")
                    .about("Gets the incoming changes from the current git repository.")
                    .arg(arg!([PATH] "Path to the git repository.")
                        .required(false))
                    .aliases(&["gi", "incoming", "in"]))
            .get_matches();

    match matches.subcommand() {
        Some(("FixPathSeparators", sub_matches)) => {
            let path = 
                if sub_matches.is_present("PATH") {
                    Some(sub_matches.value_of("PATH").unwrap().to_string())
                } else {
                    None
                };
            FixPathSeparatorsCommand::new(path).execute()?
        },
        Some(("GitIncoming", sub_matches)) => {
            let path = 
                if sub_matches.is_present("PATH") {
                    Some(sub_matches.value_of("PATH").unwrap().to_string())
                } else {
                    None
                };
            match GitIncomingCommand::new(path).execute() {
                Ok(_) => (),
                Err(_) => return Err(()),
            }
        },
        _ => (),
    }

    Ok(())
}
