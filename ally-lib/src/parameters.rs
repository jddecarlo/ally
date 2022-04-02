use clap::{Command, Arg, ArgMatches};
use yaml_rust::YamlLoader;

pub fn parse_command_line_args_from_yaml_string(yaml_string: &str) -> Result<ArgMatches, i32> {
    let docs = YamlLoader::load_from_str(yaml_string).map_err(| _e | { 0 })?;
    let doc = &docs[0];
    let command_data = &doc["command"];
    
    let mut cmd = Command::new(command_data["name"].as_str().ok_or(0)?);
    
    let args_data = command_data["args"].as_vec().ok_or(0)?;
    for arg_data in args_data {
        let arg_properties = &arg_data["arg"];
        let mut arg = Arg::new(arg_properties["name"].as_str().ok_or(0)?);

        let short_arg = &arg_properties["short"];
        if !short_arg.is_badvalue() {
            arg = arg.short(short_arg.as_str().ok_or(0)?.chars().nth(0).ok_or(0)?);
        }

        let long_arg = &arg_properties["long"];
        if !long_arg.is_badvalue() {
            arg = arg.long(long_arg.as_str().ok_or(0)?);
        }

        cmd = cmd.arg(arg);
    }
    
    Ok(cmd.get_matches())
}