use clap::{Command, Arg, ArgMatches};
use yaml_rust::YamlLoader;

pub fn parse_command_line_args_from_yaml_string(yaml_string: &str) -> ArgMatches {
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];
    let command_data = &doc["command"];
    
    let mut cmd = Command::new(command_data["name"].as_str().unwrap());
    
    let args_data = command_data["args"].as_vec().unwrap();
    for arg_data in args_data {
        let arg_properties = &arg_data["arg"];
        let mut arg = Arg::new(arg_properties["name"].as_str().unwrap());

        if !arg_properties["short"].is_badvalue() {
            arg = arg.short(arg_properties["short"].as_str().unwrap().chars().next().unwrap().to_owned());
        }

        if !arg_properties["long"].is_badvalue() {
            arg = arg.long(arg_properties["long"].as_str().unwrap());
        }

        cmd = cmd.arg(arg);
    }
    
    cmd.get_matches()
}