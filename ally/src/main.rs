use std::include_str;
use ally_lib::parameters;

fn main() {
    let yaml_string = include_str!("args.yaml");

    let matches = parameters::parse_command_line_args_from_yaml_string(yaml_string);
    println!("{:?}", matches);
}
