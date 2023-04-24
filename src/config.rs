use std::{collections::HashMap, fs};

pub fn get() -> HashMap<String, String> {
    let configs = fs::read_to_string(".env").unwrap();

    let parse: Vec<&str> = configs.split("\n").collect();

    let mut final_configuration: HashMap<String, String> = HashMap::new();

    for conf in parse {
        let split: Vec<&str> = conf.split("=").collect();
        final_configuration.insert(String::from(split[0]), String::from(split[1]));
    }

    return final_configuration;
}
