use crate::plugins::{PluginList, Plugin};
use std::collections::HashMap;
use std::process::Command;
use std::io::prelude::*;

use super::functions::time;

// each plugin has its own string output. 
// however, since we are using multiple threads we will need
// a map so we can only update the correct plugin 
pub fn generate_map(plg_list : &PluginList) -> HashMap<&String, String> {
    let mut map = HashMap::new();
    for plugin in &plg_list.items {
        // initialized with blank strings since no output
        map.insert(&plugin.name, String::from(""));
    }
    map
}

pub fn xsetroot(output: &str) -> () {
    let xset_result = Command::new("xsetroot")
        .arg("-name")
        .arg(output.to_string())
        .spawn();
    if let Err(_) = xset_result {
            writeln!(&super::logging::setup_logger(), "xsetroot failed at {}", super::functions::time())
            .expect("writing to log failed");
    };
}