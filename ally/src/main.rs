use std::include_str;
use yaml_rust::YamlLoader;
use ally_lib::command::Command;

fn main() {
    let yaml_string = include_str!("args.yaml");
    let docs = YamlLoader::load_from_str(yaml_string).unwrap();
    let doc = &docs[0];

    let command = Command::new(doc);
    println!("{:?}", command);
}
