use crate::plugins::{PluginList, Plugin};
use std::collections::HashMap;



// each plugin has its own string output. 
// however, since we are using multiple threads we will need
// a map so we can only update the correct plugin 
pub fn generate_map(plg_list : &PluginList) -> HashMap<&Plugin, String> {
    let mut map = HashMap::new();
    for plugin in &plg_list.items {
        // initialized with blank strings since no output
        map.insert(plugin, String::from(""));
    }
    map
}