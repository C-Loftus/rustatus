pub mod plugins;
pub mod functions;
pub mod logging;
pub mod output;

use {logging::*, plugins::{Plugin, PluginList}};

use std::thread;
use std::io::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use gag::Redirect;
use output::xsetroot;
#[macro_use]
extern crate lazy_static;

const CONFIG_PATH: &str = "src/config.yaml";

fn main() {
    // read config and create a list of associated functions
    let plugin_list = PluginList::new(CONFIG_PATH);

    let mut output_map: HashMap<String, String> = output::generate_map(&plugin_list);
    // panics are logged to home dir
    let log = setup_logger();
    Redirect::stderr(log).unwrap(); 


    let data = Arc::new(Mutex::new(output_map.to_owned()));
    
    loop {
        // base string
        // concatenate all output strings
        // for plg in &plugin_list.items {
        //     output += &(plg.associated_fn)();
        // }
        for plg in &plugin_list.items {
            let data = Arc::clone(&data);
            let data_handle = thread::spawn( move || {
                    let mut string = data.lock().unwrap();
                    // let o = string.entry(&plg.name).or_insert((plg.associated_fn)());
                    // *o = (plg.associated_fn)();
                }
            );


        }
        
        // output::xsetroot(&output);
    }
}
