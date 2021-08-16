
#![allow(dead_code)]
// my crates
use super::functions::*;
// std crates
use std::collections::HashMap;
// external crates
extern crate yaml_rust;
use yaml_rust::{YamlLoader};

const CONFIG_PATH: &str = "src/config.yaml";

// in order to get a const map you have to wrap it in a lazy static
// rust doesn't have literal maps and using a macro would introduce an uncessary lifetime
lazy_static! {
    static ref MAP: HashMap<String, fn() -> String> = {
        let mut m: HashMap<String, fn() -> String> = HashMap::new();
        m.insert(String::from("Volume"), volume);
        m.insert(String::from("Network"), network_name);
        m.insert(String::from("Time"), time);
        m.insert(String::from("Mouse Battery"), mouse_battery);
        m.insert(String::from("Battery"), internal_battery);
        m
    };
}


pub struct PluginList {
    items: Vec<Plugin>,
} 

pub struct Plugin {
    pub associated_fn: fn() -> String,
    pub rate: Option<u8>,
}

impl Plugin {
    fn get_rate() -> Option<u8> {

        None
    }
}

impl PluginList {
    fn new() -> Self {

        let mut returned_list =  Self {
            items : Vec::new(),
        };

        // parse yaml data
        let raw = &std::fs::read_to_string(CONFIG_PATH).unwrap();
        let yml = YamlLoader::load_from_str(raw).unwrap();
        let config = &yml[0];
        let plugin_list = &config["modules"];

        // iterate over yaml data and insert the corresponding functions into each plugin 
        // struct. Then put that plugin into the list of all our plugins
        for plugin in plugin_list.as_vec().unwrap() {
            let fn_pointer = MAP[plugin.as_str().unwrap()]; 
            let inserted_plugin = Plugin {
                associated_fn: fn_pointer,
                // temp code
                rate : None
            };
            returned_list.add_plugin(inserted_plugin)
        }
        returned_list
    }

    fn add_plugin (&mut self, p: Plugin) -> () {
        self.items.push(p);
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_parse() {
        let raw = &std::fs::read_to_string(CONFIG_PATH).unwrap();
        let yml = YamlLoader::load_from_str(raw).unwrap();
        let config = &yml[0];
        let plugin_list = &config["modules"];
        for plugin in plugin_list.as_vec().unwrap() {
            print!("{}\n", plugin.as_str().unwrap());
        }
    }
    #[test]
    fn test_plugin_list() {
        // should match the output for the functions
        // you placed in config.yaml
        let list = PluginList::new();
        for plugin in list.items {
            print!("{:?}\n", (plugin.associated_fn)())
        }

    }

    #[test]
    fn test_map() {
        let volume_pointer = (MAP[&String::from("Volume")])();
        assert_eq!(volume_pointer, volume())
    }
}
