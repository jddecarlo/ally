use std::cell::RefCell;
use clap::{Command, Arg, ArgMatches};
use yaml_rust::{YamlLoader, Yaml};

pub fn parse_command_line_args_from_yaml_string(yaml_string: &str) -> Result<ArgMatches, i32> {
    let docs = YamlLoader::load_from_str(yaml_string).map_err(| _e | { 0 })?;
    let doc = &docs[0];
    let command_data = &doc["command"];
    
    let mut cmd = Command::new(command_data["name"].as_str().ok_or(0)?);
    
    let args_data = command_data["args"].as_vec().ok_or(0)?;
    for arg_data in args_data {
        let arg_properties = &arg_data["arg"];
        let arg = RefCell::new(Arg::new(arg_properties["id"].as_str().ok_or(0)?));

        set_arg_property(&arg, &arg_properties["alias"], Arg::alias, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["allow_hyphen_values"], Arg::allow_hyphen_values, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["allow_invalid_utf8"], Arg::allow_invalid_utf8, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["conflicts_with"], Arg::conflicts_with, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["default_missing_value"], Arg::default_missing_value, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["default_value"], Arg::default_value, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["display_order"], Arg::display_order, | property | {
            if let Some(i) = property.as_i64() {
                Some(i as usize)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["env"], Arg::env, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["exclusive"], Arg::exclusive, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["forbid_empty_values"], Arg::forbid_empty_values, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["global"], Arg::global, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["group"], Arg::group, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["help"], Arg::help, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["hide"], Arg::hide, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["hide_default_value"], Arg::hide_default_value, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["hide_env"], Arg::hide_env, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["hide_env_values"], Arg::hide_env_values, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["hide_long_help"], Arg::hide_long_help, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["hide_possible_values"], Arg::hide_possible_values, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["hide_short_help"], Arg::hide_short_help, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["ignore_case"], Arg::ignore_case, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["index"], Arg::index, | property | {
            if let Some(i) = property.as_i64() {
                Some(i as usize)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["last"], Arg::last, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["long"], Arg::long, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["long_help"], Arg::long_help, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["max_occurrences"], Arg::max_occurrences, | property | {
            if let Some(i) = property.as_i64() {
                Some(i as usize)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["max_values"], Arg::max_values, | property | {
            if let Some(i) = property.as_i64() {
                Some(i as usize)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["min_values"], Arg::min_values, | property | {
            if let Some(i) = property.as_i64() {
                Some(i as usize)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["multiple_occurrences"], Arg::multiple_occurrences, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["multiple_values"], Arg::multiple_values, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["next_line_help"], Arg::next_line_help, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["number_of_values"], Arg::number_of_values, | property | {
            if let Some(i) = property.as_i64() {
                Some(i as usize)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["overrides_with"], Arg::overrides_with, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["possible_value"], Arg::possible_value, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["raw"], Arg::raw, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["require_equals"], Arg::require_equals, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["require_value_delimiter"], Arg::require_value_delimiter, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["required"], Arg::required, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["required_unless_present"], Arg::required_unless_present, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["requires"], Arg::requires, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["short"], Arg::short, | property | {
            if let Some(s) = property.as_str() {
                s.chars().nth(0)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["short_alias"], Arg::short_alias, | property | {
            if let Some(s) = property.as_str() {
                s.chars().nth(0)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["takes_value"], Arg::takes_value, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["use_value_delimiter"], Arg::use_value_delimiter, Yaml::as_bool)?;
        set_arg_property(&arg, &arg_properties["value_delimiter"], Arg::value_delimiter, | property | {
            if let Some(s) = property.as_str() {
                s.chars().nth(0)
            } else {
                None
            }
        })?;
        set_arg_property(&arg, &arg_properties["value_name"], Arg::value_name, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["value_terminator"], Arg::value_terminator, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["visible_alias"], Arg::visible_alias, Yaml::as_str)?;
        set_arg_property(&arg, &arg_properties["visible_short_alias"], Arg::visible_short_alias, | property | {
            if let Some(s) = property.as_str() {
                s.chars().nth(0)
            } else {
                None
            }
        })?;

        cmd = cmd.arg(arg.borrow().to_owned());
    }
    
    Ok(cmd.get_matches())
}

fn set_arg_property<'a, FnSetArgVal, FnGetPropVal, TProp>(arg: &RefCell<Arg<'a>>, property: &'a Yaml, set_arg_val: FnSetArgVal, get_property_val: FnGetPropVal) -> Result<(), i32>
    where FnSetArgVal: FnOnce(Arg<'a>, TProp) -> Arg<'a>,
    FnGetPropVal: FnOnce(&'a Yaml) -> Option<TProp> {
    if !property.is_badvalue() {
        let owned_arg = arg.borrow().to_owned();
        *arg.borrow_mut() = set_arg_val(owned_arg, get_property_val(property).ok_or(0)?);
    }

    Ok(())
}
