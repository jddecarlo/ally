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

        let value_name_arg = &arg_properties["value_name"];
        if !value_name_arg.is_badvalue() {
            arg = arg.value_name(value_name_arg.as_str().ok_or(0)?);
        }

        let help_arg = &arg_properties["help"];
        if !help_arg.is_badvalue() {
            arg = arg.help(help_arg.as_str().ok_or(0)?);
        }

        let long_help_arg = &arg_properties["long_help"];
        if !long_help_arg.is_badvalue() {
            arg = arg.long_help(long_help_arg.as_str().ok_or(0)?);
        }

        let takes_value_arg = &arg_properties["takes_value"];
        if !takes_value_arg.is_badvalue() {
            arg = arg.takes_value(takes_value_arg.as_bool().ok_or(0)?);
        }

        let index_arg = &arg_properties["index"];
        if !index_arg.is_badvalue() {
            arg = arg.index(index_arg.as_i64().ok_or(0)? as usize);
        }

        let hide_arg = &arg_properties["hide"];
        if !hide_arg.is_badvalue() {
            arg = arg.hide(hide_arg.as_bool().ok_or(0)?);
        }

        let conflicts_with_arg = &arg_properties["conflicts_with"];
        if !conflicts_with_arg.is_badvalue() {
            arg = arg.conflicts_with(conflicts_with_arg.as_str().ok_or(0)?);
        }

        cmd = cmd.arg(arg);
    }
    
    Ok(cmd.get_matches())
}
