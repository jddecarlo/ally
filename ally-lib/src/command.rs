use clap::Command as ClapCommand;
use clap::Arg as ClapArg;
use clap::ArgGroup as ClapArgGroup;
use yaml_rust::Yaml;

macro_rules! set_prop_str {
    ($cmd:ident, $prop:expr, $set_prop:ident) => {
        if !$prop.is_badvalue() {
            $cmd = $cmd.$set_prop($prop.as_str().unwrap());
        }
    };
}

macro_rules! set_prop_char {
    ($cmd:ident, $prop:expr, $set_prop:ident) => {
        if !$prop.is_badvalue() {
            $cmd = $cmd.$set_prop($prop.as_str().unwrap().chars().nth(0).unwrap());
        }
    };
}

macro_rules! set_prop_bool {
    ($cmd:ident, $prop:expr, $set_prop:ident) => {
        match &$prop {
            Yaml::Boolean(b) => $cmd = $cmd.$set_prop(*b),
            Yaml::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "yes" | "y" | "on" => $cmd = $cmd.$set_prop(true),
                    "false" | "no" | "n" | "off" => $cmd = $cmd.$set_prop(false),
                    _ => (),
                }
            }
            _ => (),
        }
    };
}

macro_rules! set_prop_usize {
    ($cmd:ident, $prop:expr, $set_prop:ident) => {
        if !$prop.is_badvalue() {
            $cmd = $cmd.$set_prop($prop.as_i64().unwrap() as usize);
        }
    };
}

#[derive(Debug)]
pub struct Command<'a> {
    name: String,
    command: ClapCommand<'a>,
    subcommands: Vec<Command<'a>>,
}

impl<'a> Command<'a> {
    pub fn new(config: &Yaml) -> Command {
        let command;
        if !config["command"].is_badvalue() {
            command = Command::build_command_from_yaml(&config["command"]);
        } else {
            command = Command::build_command_from_yaml(config);
        }
        let name = command.get_name().to_string();
        let subcommands = Command::extract_subcommands(&command);
        
        Command {
            name,
            command,
            subcommands,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_command(&self) -> &ClapCommand<'a> {
        &self.command
    }

    pub fn get_subcommands(&self) -> &Vec<Command<'a>> {
        &self.subcommands
    }

    fn extract_subcommands(command: &ClapCommand<'a>) -> Vec<Command<'a>> {
        let mut subcommands : Vec<Command<'a>> = Vec::new();
        for clap_subcommand in command.get_subcommands() {
            let name = clap_subcommand.get_name().to_string();
            let sub_subcommands = Command::extract_subcommands(&clap_subcommand);
            let subcommand = Command {
                name,
                command: clap_subcommand.clone(),
                subcommands: sub_subcommands,
            };
            subcommands.push(subcommand);
        }

        subcommands
    }

    fn build_command_from_yaml(cmd_properties: &'a Yaml) -> ClapCommand<'a> {
        let mut cmd = ClapCommand::new(cmd_properties["name"].as_str().unwrap().to_string());

        // Command settings
        set_prop_str!(cmd, cmd_properties["about"], about);
        set_prop_str!(cmd, cmd_properties["after_help"], after_help);
        set_prop_str!(cmd, cmd_properties["after_long_help"], after_long_help);
        set_prop_str!(cmd, cmd_properties["alias"], alias);
        set_prop_bool!(cmd, cmd_properties["allow_external_subcommands"], allow_external_subcommands);
        set_prop_bool!(cmd, cmd_properties["allow_hyphen_values"], allow_hyphen_values);
        set_prop_bool!(cmd, cmd_properties["allow_invalid_utf8_for_external_subcommands"], allow_invalid_utf8_for_external_subcommands);
        set_prop_bool!(cmd, cmd_properties["allow_missing_positional"], allow_missing_positional);
        set_prop_bool!(cmd, cmd_properties["allow_negative_numbers"], allow_negative_numbers);
        set_prop_bool!(cmd, cmd_properties["arg_required_else_help"], arg_required_else_help);
        set_prop_bool!(cmd, cmd_properties["args_conflicts_with_subcommands"], args_conflicts_with_subcommands);
        set_prop_bool!(cmd, cmd_properties["args_override_self"], args_override_self);
        set_prop_str!(cmd, cmd_properties["author"], author);
        set_prop_str!(cmd, cmd_properties["before_help"], before_help);
        set_prop_str!(cmd, cmd_properties["before_long_help"], before_long_help);
        set_prop_str!(cmd, cmd_properties["bin_name"], bin_name);
        set_prop_bool!(cmd, cmd_properties["disable_colored_help"], disable_colored_help);
        set_prop_bool!(cmd, cmd_properties["disable_help_flag"], disable_help_flag);
        set_prop_bool!(cmd, cmd_properties["disable_help_subcommand"], disable_help_subcommand);
        set_prop_bool!(cmd, cmd_properties["disable_version_flag"], disable_version_flag);
        set_prop_usize!(cmd, cmd_properties["display_order"], display_order);
        set_prop_bool!(cmd, cmd_properties["dont_collapse_args_in_usage"], dont_collapse_args_in_usage);
        set_prop_bool!(cmd, cmd_properties["dont_delimit_trailing_values"], dont_delimit_trailing_values);
        set_prop_bool!(cmd, cmd_properties["help_expected"], help_expected);
        set_prop_str!(cmd, cmd_properties["help_template"], help_template);
        set_prop_bool!(cmd, cmd_properties["hide"], hide);
        set_prop_bool!(cmd, cmd_properties["hide_possible_values"], hide_possible_values);
        set_prop_bool!(cmd, cmd_properties["ignore_errors"], ignore_errors);
        set_prop_bool!(cmd, cmd_properties["infer_long_args"], infer_long_args);
        set_prop_bool!(cmd, cmd_properties["infer_subcommands"], infer_subcommands);
        set_prop_str!(cmd, cmd_properties["long_about"], long_about);
        set_prop_str!(cmd, cmd_properties["long_flag"], long_flag);
        set_prop_str!(cmd, cmd_properties["long_flag_alias"], long_flag_alias);
        set_prop_str!(cmd, cmd_properties["long_version"], long_version);
        set_prop_usize!(cmd, cmd_properties["max_term_width"], max_term_width);
        set_prop_str!(cmd, cmd_properties["next_help_heading"], next_help_heading);
        set_prop_bool!(cmd, cmd_properties["next_line_help"], next_line_help);
        set_prop_bool!(cmd, cmd_properties["no_binary_name"], no_binary_name);
        set_prop_str!(cmd, cmd_properties["override_help"], override_help);
        set_prop_str!(cmd, cmd_properties["override_usage"], override_usage);
        set_prop_bool!(cmd, cmd_properties["propagate_version"], propagate_version);
        set_prop_char!(cmd, cmd_properties["short_flag"], short_flag);
        set_prop_char!(cmd, cmd_properties["short_flag_alias"], short_flag_alias);
        set_prop_str!(cmd, cmd_properties["subcommand_help_heading"], subcommand_help_heading);
        set_prop_bool!(cmd, cmd_properties["subcommand_negates_reqs"], subcommand_negates_reqs);
        set_prop_bool!(cmd, cmd_properties["subcommand_precedence_over_arg"], subcommand_precedence_over_arg);
        set_prop_bool!(cmd, cmd_properties["subcommand_required"], subcommand_required);
        set_prop_str!(cmd, cmd_properties["subcommand_value_name"], subcommand_value_name);
        set_prop_usize!(cmd, cmd_properties["term_width"], term_width);
        set_prop_bool!(cmd, cmd_properties["trailing_var_arg"], trailing_var_arg);
        set_prop_str!(cmd, cmd_properties["version"], version);
        set_prop_str!(cmd, cmd_properties["visible_alias"], visible_alias);
        set_prop_str!(cmd, cmd_properties["visible_long_flag_alias"], visible_long_flag_alias);
        set_prop_char!(cmd, cmd_properties["visible_short_flag_alias"], visible_short_flag_alias);
        
        // Arg settings
        if let Some(args_data) = cmd_properties["args"].as_vec() {
            for arg_data in args_data {
                let arg_properties = &arg_data["arg"];
                let arg = Command::parse_arg_settings(arg_properties);
                cmd = cmd.arg(arg);
            }
        }

        // ClapArgGroup settings
        if let Some(groups_data) = cmd_properties["groups"].as_vec() {
            for group_data in groups_data {
                let group_properties = &group_data["group"];
                let group = Command::parse_group_settings(group_properties);
                cmd = cmd.group(group);
            }
        }

        // Subcommands
        if let Some(subcommands_data) = cmd_properties["subcommands"].as_vec() {
            for subcommand_data in subcommands_data {
                let subcommand_properties = &subcommand_data["command"];
                let subcommand = Command::build_command_from_yaml(subcommand_properties);
                cmd = cmd.subcommand(subcommand);
            }
        }

        cmd
    }

    fn parse_arg_settings(arg_properties: &'a Yaml) -> ClapArg<'a> {
        let mut arg = ClapArg::new(arg_properties["id"].as_str().unwrap());

        set_prop_str!(arg, arg_properties["alias"], alias);
        set_prop_bool!(arg, arg_properties["allow_hyphen_values"], allow_hyphen_values);
        set_prop_bool!(arg, arg_properties["allow_invalid_utf8"], allow_invalid_utf8);
        set_prop_str!(arg, arg_properties["conflicts_with"], conflicts_with);
        set_prop_str!(arg, arg_properties["default_missing_value"], default_missing_value);
        set_prop_str!(arg, arg_properties["default_value"], default_value);
        set_prop_usize!(arg, arg_properties["display_order"], display_order);
        set_prop_str!(arg, arg_properties["env"], env);
        set_prop_bool!(arg, arg_properties["exclusive"], exclusive);
        set_prop_bool!(arg, arg_properties["forbid_empty_values"], forbid_empty_values);
        set_prop_bool!(arg, arg_properties["global"], global);
        set_prop_str!(arg, arg_properties["group"], group);
        set_prop_str!(arg, arg_properties["help"], help);
        set_prop_bool!(arg, arg_properties["hide"], hide);
        set_prop_bool!(arg, arg_properties["hide_default_value"], hide_default_value);
        set_prop_bool!(arg, arg_properties["hide_env"], hide_env);
        set_prop_bool!(arg, arg_properties["hide_env_values"], hide_env_values);
        set_prop_bool!(arg, arg_properties["hide_long_help"], hide_long_help);
        set_prop_bool!(arg, arg_properties["hide_possible_values"], hide_possible_values);
        set_prop_bool!(arg, arg_properties["hide_short_help"], hide_short_help);
        set_prop_bool!(arg, arg_properties["ignore_case"], ignore_case);
        set_prop_usize!(arg, arg_properties["index"], index);
        set_prop_bool!(arg, arg_properties["last"], last);
        set_prop_str!(arg, arg_properties["long"], long);
        set_prop_str!(arg, arg_properties["long_help"], long_help);
        set_prop_usize!(arg, arg_properties["max_occurrences"], max_occurrences);
        set_prop_usize!(arg, arg_properties["max_values"], max_values);
        set_prop_usize!(arg, arg_properties["min_values"], min_values);
        set_prop_bool!(arg, arg_properties["multiple_occurrences"], multiple_occurrences);
        set_prop_bool!(arg, arg_properties["multiple_values"], multiple_values);
        set_prop_bool!(arg, arg_properties["next_line_help"], next_line_help);
        set_prop_usize!(arg, arg_properties["number_of_values"], number_of_values);
        set_prop_str!(arg, arg_properties["overrides_with"], overrides_with);
        set_prop_str!(arg, arg_properties["possible_value"], possible_value);
        set_prop_bool!(arg, arg_properties["raw"], raw);
        set_prop_bool!(arg, arg_properties["require_equals"], require_equals);
        set_prop_bool!(arg, arg_properties["require_value_delimiter"], require_value_delimiter);
        set_prop_bool!(arg, arg_properties["required"], required);
        set_prop_str!(arg, arg_properties["required_unless_present"], required_unless_present);
        set_prop_str!(arg, arg_properties["requires"], requires);
        set_prop_char!(arg, arg_properties["short"], short);
        set_prop_char!(arg, arg_properties["short_alias"], short_alias);
        set_prop_bool!(arg, arg_properties["takes_value"], takes_value);
        set_prop_bool!(arg, arg_properties["use_value_delimiter"], use_value_delimiter);
        set_prop_char!(arg, arg_properties["value_delimiter"], value_delimiter);
        set_prop_str!(arg, arg_properties["value_name"], value_name);
        set_prop_str!(arg, arg_properties["value_terminator"], value_terminator);
        set_prop_str!(arg, arg_properties["visible_alias"], visible_alias);
        set_prop_char!(arg, arg_properties["visible_short_alias"], visible_short_alias);
    
        arg
    }
    
    fn parse_group_settings(group_properties: &'a Yaml) -> ClapArgGroup<'a> {
        let mut group = ClapArgGroup::new(group_properties["id"].as_str().unwrap());
    
        set_prop_str!(group, group_properties["arg"], arg);
        set_prop_str!(group, group_properties["conflicts_with"], conflicts_with);
        set_prop_bool!(group, group_properties["multiple"], multiple);
        set_prop_bool!(group, group_properties["required"], required);
        set_prop_str!(group, group_properties["requires"], requires);

        group
    }
}
