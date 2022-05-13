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
                    .aliases(&["gi", "incoming", "in"]))
            .subcommand(
                Command::new("GitOutgoing")
                    .about("Gets the outgoing changes from the current git repository.")
                    .aliases(&["go", "outgoing", "out"]))
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
        Some(("GitIncoming", _sub_matches)) => {
            match GitIncomingCommand::new().execute() {
                Ok(_) => (),
                Err(_) => return Err(()),
            }
        },
        Some(("GitOutgoing", _sub_matches)) => {
            match GitOutgoingCommand::new().execute() {
                Ok(_) => (),
                Err(_) => return Err(()),
            }
        },
        _ => (),
    }

    Ok(())
}
