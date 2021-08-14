
#![allow(dead_code)]

use super::functions::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;
use std::fs::read_to_string;
use std::collections::HashMap;



const CONFIG_PATH: &str = "src/config.json";

lazy_static! {
    static ref MAP: HashMap<String, fn() -> String> = {
        let mut t: HashMap<String, fn() -> String> = HashMap::new();
        t.insert(String::from("Volume"), volume);
        t
    };
}


pub struct PluginList {
    items: Vec<&'static Plugin>,
} 

pub struct Plugin {
    pub get_data: &'static dyn Fn(),
    pub rate: u8,
}

// potentially switch to enum map?
/// hashmap literal doesn't support &dyn Fn with 
/// different fn so it needs to be wrapped
enum FuncWrapper {
    New(&'static dyn Fn() -> String), 
}


impl Plugin {
    fn plugin_map() -> () {

        
    }
}

impl PluginList {
    fn new() -> Self {

        let returned_list =  Self {
            items : Vec::new(),
        };
        let raw = &std::fs::read_to_string(CONFIG_PATH).unwrap();
        let js = json::parse(raw).unwrap();
        for plugin in js.members() {
            let parsed_plugin = MAP[&plugin.to_string()];

            // returned_list.add_plugin()
        }

        
            
        returned_list
    }

    fn add_plugin (&mut self, m: &'static Plugin) -> () {
        self.items.push(m);
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config() {
        let raw = &std::fs::read_to_string("src/config.json").unwrap();
        let js = json::parse(raw).unwrap();
        assert!(js["modules"].is_array());
        assert!(js["this"]["does"]["not"]["exist"].is_null());

    }

    #[test]
    fn test_map() {
        let volume_pointer = (MAP[&String::from("Volume")])();
        assert_eq!(volume_pointer, volume())
    }
}
