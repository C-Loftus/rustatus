
#![allow(dead_code)]
// my crates
use super::functions::*;
// std crates
use std::collections::HashMap;
use std::time::Duration;
// external crates
extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};


// in order to get a const map you have to wrap it in a lazy static
// rust doesn't have map literals
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

/* const terms used when parsing config file */
const MODULES: &'static str = "Modules";
const REFRESH_RATES: &'static str = "Refresh_Rates";
// yaml parser can read multiple files so we this specifies
// 0 for the only index in the yaml file array
const ONLY_YAML_INDEX: usize = 0;


pub struct PluginList {
    pub items: Vec<Plugin>,
} 

#[derive(Hash, Debug)]
pub struct Plugin {
    pub associated_fn: fn() -> String,
    pub rate: Option<Duration>,
    pub name: String,
}



/// Needed for hash table insertion
impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.associated_fn == other.associated_fn
    }
}
impl Eq for Plugin {}


fn get_rate(fn_to_match: &str, yml: &Yaml) -> Option<Duration> {
    let rate_list = &(yml[REFRESH_RATES]);
    for rate in rate_list.as_hash().unwrap() {
        if let Some(valid_string) = rate.0.as_str() {
            if valid_string == fn_to_match {
                // yaml crate only supports parsing as i64
                // so we need to do manual cast for duration
                // this panics on invalid parse
                let rate = rate.1.as_i64().unwrap() as u64;
                let duration = Duration::from_millis(rate);
                return Some(duration)
            }
        }
    }
    // return nothing if no duration specified
    None
}


impl PluginList {
    pub fn new(config_filename: &str) -> Self {

        let mut returned_list =  Self {
            items : Vec::new(),
        };

        // parse yaml data
        let raw = std::fs::read_to_string(config_filename).unwrap();
        let yml = YamlLoader::load_from_str(&raw).unwrap();
        let config = &yml[ONLY_YAML_INDEX];
        let plugin_list = &config[MODULES];

        // iterate over yaml data and insert the corresponding functions into each plugin 
        // struct. Then put that plugin into the list of all our plugins
        for plugin in plugin_list.as_vec().unwrap() {
            let plugin_name = plugin.as_str().unwrap();
            let fn_pointer = MAP[plugin_name]; 
            let rate = get_rate(plugin_name, config);

            returned_list.add_plugin(fn_pointer, rate, plugin_name.to_string());
        }
        returned_list
    }

    fn add_plugin (&mut self, associated_fn: fn() -> String, rate : Option<Duration>, name: String) -> () {
        let inserted_plugin = Plugin {
            associated_fn,
            rate,
            name,
        };
        self.items.push(inserted_plugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_CONFIG_PATH: &str = "examples/sample_config.yaml";

    #[test]
    fn test_config_parse() {
        let raw = std::fs::read_to_string(SAMPLE_CONFIG_PATH).unwrap();
        let yml = YamlLoader::load_from_str(&raw).unwrap();
        let config = &yml[0];
        let plugin_list = &config["Modules"];
        for plugin in plugin_list.as_vec().unwrap() {
            print!("Plugin Parse Test : {}\n", plugin.as_str().unwrap());
        }
    }
    #[test]
    fn test_plugin_list() {
        // should match the output for the functions
        // you placed in config.yaml
        let list = PluginList::new(SAMPLE_CONFIG_PATH);
        for plugin in list.items {
            print!("Function Pointer : {:?}\n", (plugin.associated_fn)())
        }

    }
    #[test]
    fn test_associative_values() {
        let raw = std::fs::read_to_string(SAMPLE_CONFIG_PATH).unwrap();
        let yml = YamlLoader::load_from_str(&raw).unwrap();
        let config = &yml[0];
        let plugin_list = &config["Refresh_Rates"];
        println!("{:?}", plugin_list);
        for plugin in plugin_list.as_hash().unwrap() {
            print!("Function {:?} , Update Rate : {:?}\n", plugin.0.as_str().unwrap(), plugin.1.as_i64().unwrap());
        }
    }


    #[test]
    fn test_map_name_to_fn() {
        let volume_pointer = (MAP[&String::from("Volume")])();
        assert_eq!(volume_pointer, volume())
    }
    #[test]
    fn test_plugin_print() {
        let p = Plugin {
            associated_fn: volume,
            rate: None,
            name: String::from("test"),
        };
        print!("{:?}{:#?}", p, p.associated_fn);
    }
}
