pub mod plugins;
pub mod functions;
pub mod logging;
pub mod output;

use {logging::*, plugins::{Plugin, PluginList}};

use std::{thread, collections::HashMap};
use std::sync::{Arc, Mutex};

use gag::Redirect;
#[macro_use]
extern crate lazy_static;

const CONFIG_PATH: &str = "src/config.yaml";



fn main() {
    // read config and create a list of associated functions
    let plugin_list = PluginList::new(CONFIG_PATH);

    let output_map: HashMap<String, String> = output::generate_map(&plugin_list);
    // panics are logged to home dir
    let log = setup_logger();
    Redirect::stderr(log).unwrap(); 


    let data = Arc::new(Mutex::new(output_map.to_owned()));

    // boxed slice can be passed into the thread multiple times    
    let plugin_array: Box<[Plugin]> = plugin_list.items.into_boxed_slice();

    loop {

        for plg in plugin_array.to_vec() {
            
            let data = Arc::clone(&data);

            let _data_handle = thread::spawn( move || {
                    // get the map that stores all the output
                    let mut map = data.lock().unwrap();
                    // update the map value within the respective plugin
                    let map_val = map.entry(plg.name.to_owned()).or_insert((plg.associated_fn)());
                    *map_val = (plg.associated_fn)();
                    // drops the map so it unlocks for other threads
                    drop(map);
                    if let Some(duration) = plg.rate {
                        thread::sleep(duration);
                    }
                }
            );


        }
        // after loop finishes and gets all plugin values, xsetroot 
        // with the map values
        let data = Arc::clone(&data);
        let borrowed_map = &*(data.lock().unwrap());
        let mut output_string = String::from("");
        for (_, value) in borrowed_map.into_iter() {
            output_string += &value;
        }
        output::xsetroot(&output_string);
    }
}
