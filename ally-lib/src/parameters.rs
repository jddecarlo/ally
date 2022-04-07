use std::cell::RefCell;
use clap::{Command, Arg, ArgMatches};
use yaml_rust::{YamlLoader, Yaml};

pub fn parse_command_line_args_from_yaml_string(yaml_string: &str) -> Result<ArgMatches, i32> {
    let docs = YamlLoader::load_from_str(yaml_string).map_err(|_e| { 0 })?;
    let doc = &docs[0];
    let cmd = parse_cmd_settings(doc)?;
    let owned_cmd = cmd.borrow().to_owned();
    Ok(owned_cmd.get_matches())
}

fn parse_cmd_settings(doc: &Yaml) -> Result<RefCell<Command>, i32> {
    let cmd_properties = &doc["command"];

    // Command settings
    let cmd = RefCell::new(Command::new(cmd_properties["name"].as_str().ok_or(0)?));

    set_cmd_property(&cmd, &cmd_properties["about"], Command::about, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["after_help"], Command::after_help, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["after_help"], Command::after_help, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["after_long_help"], Command::after_long_help, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["alias"], Command::alias, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["allow_external_subcommands"], Command::allow_external_subcommands, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["allow_hyphen_values"], Command::allow_hyphen_values, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["allow_invalid_utf8_for_external_subcommands"], Command::allow_invalid_utf8_for_external_subcommands, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["allow_missing_positional"], Command::allow_missing_positional, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["allow_negative_numbers"], Command::allow_negative_numbers, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["arg_required_else_help"], Command::arg_required_else_help, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["args_conflicts_with_subcommands"], Command::args_conflicts_with_subcommands, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["args_override_self"], Command::args_override_self, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["author"], Command::author, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["before_help"], Command::before_help, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["before_long_help"], Command::before_long_help, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["bin_name"], Command::bin_name, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["disable_colored_help"], Command::disable_colored_help, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["disable_help_flag"], Command::disable_help_flag, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["disable_help_subcommand"], Command::disable_help_subcommand, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["disable_version_flag"], Command::disable_version_flag, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["display_order"], Command::display_order, |property| {
        if let Some(i) = property.as_i64() {
            Some(i as usize)
        } else {
            None
        }
    })?;
    set_cmd_property(&cmd, &cmd_properties["dont_collapse_args_in_usage"], Command::dont_collapse_args_in_usage, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["dont_delimit_trailing_values"], Command::dont_delimit_trailing_values, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["help_expected"], Command::help_expected, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["help_template"], Command::help_template, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["hide"], Command::hide, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["hide_possible_values"], Command::hide_possible_values, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["ignore_errors"], Command::ignore_errors, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["infer_long_args"], Command::infer_long_args, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["infer_subcommands"], Command::infer_subcommands, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["long_about"], Command::long_about, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["long_flag"], Command::long_flag, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["long_flag_alias"], Command::long_flag_alias, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["long_version"], Command::long_version, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["max_term_width"], Command::max_term_width, |property| {
        if let Some(i) = property.as_i64() {
            Some(i as usize)
        } else {
            None
        }
    })?;
    set_cmd_property(&cmd, &cmd_properties["next_help_heading"], Command::next_help_heading, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["next_line_help"], Command::next_line_help, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["no_binary_name"], Command::no_binary_name, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["override_help"], Command::override_help, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["override_usage"], Command::override_usage, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["propagate_version"], Command::propagate_version, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["short_flag"], Command::short_flag, |property| {
        if let Some(s) = property.as_str() {
            s.chars().nth(0)
        } else {
            None
        }
    })?;
    set_cmd_property(&cmd, &cmd_properties["short_flag_alias"], Command::short_flag_alias, |property| {
        if let Some(s) = property.as_str() {
            s.chars().nth(0)
        } else {
            None
        }
    })?;
    set_cmd_property(&cmd, &cmd_properties["subcommand_help_heading"], Command::subcommand_help_heading, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["subcommand_negates_reqs"], Command::subcommand_negates_reqs, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["subcommand_precedence_over_arg"], Command::subcommand_precedence_over_arg, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["subcommand_required"], Command::subcommand_required, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["subcommand_value_name"], Command::subcommand_value_name, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["term_width"], Command::term_width, |property| {
        if let Some(i) = property.as_i64() {
            Some(i as usize)
        } else {
            None
        }
    })?;
    set_cmd_property(&cmd, &cmd_properties["trailing_var_arg"], Command::trailing_var_arg, Yaml::as_bool)?;
    set_cmd_property(&cmd, &cmd_properties["version"], Command::version, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["visible_alias"], Command::visible_alias, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["visible_long_flag_alias"], Command::visible_long_flag_alias, Yaml::as_str)?;
    set_cmd_property(&cmd, &cmd_properties["visible_short_flag_alias"], Command::visible_short_flag_alias, |property| {
        if let Some(s) = property.as_str() {
            s.chars().nth(0)
        } else {
            None
        }
    })?;

    // TODO: Add support for subcommands.

    // TODO: Add ArgGroup support, including Command.group() support..

    // Arg settings
    let args_data = cmd_properties["args"].as_vec().ok_or(0)?;
    for arg_data in args_data {
        let arg_properties = &arg_data["arg"];
        let arg = parse_arg_settings(arg_properties)?;

        let owned_arg = arg.borrow().to_owned();
        let owned_cmd = cmd.borrow().to_owned();
        cmd.replace(owned_cmd.arg(owned_arg));
    }

    Ok(cmd)
}

fn parse_arg_settings(arg_properties: &Yaml) -> Result<RefCell<Arg>, i32> {
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

    Ok(arg)
}

fn set_cmd_property<'a, FnSetCmdVal, FnGetPropVal, TProp>(cmd: &RefCell<Command<'a>>, property: &'a Yaml, set_cmd_val: FnSetCmdVal, get_property_val: FnGetPropVal) -> Result<(), i32>
    where FnSetCmdVal: FnOnce(Command<'a>, TProp) -> Command<'a>,
    FnGetPropVal: FnOnce(&'a Yaml) -> Option<TProp> {
        if !property.is_badvalue() {
            let owned_cmd = cmd.borrow().to_owned();
            *cmd.borrow_mut() = set_cmd_val(owned_cmd, get_property_val(property).ok_or(0)?);
        }

        Ok(())
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
