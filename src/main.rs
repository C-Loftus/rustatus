pub mod plugins;
pub mod functions;
pub mod logging;
pub mod output;
// my modules
use functions::*;
use logging::*;
use output::generate_map;
use plugins::{Plugin, PluginList};

use std::process::Command;
use std::thread;
use gag::Redirect;
use std::io::prelude::*;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

const CONFIG_PATH: &str = "src/config.yaml";

fn main() {
    // read config and create a list of associated functions
    let plugin_list = PluginList::new(CONFIG_PATH);

    let output_map: HashMap<&Plugin, String> = generate_map(&plugin_list);
    // panics are logged to home dir
    let log = setup_logger();
    let _ = Redirect::stderr(log).unwrap(); 

    loop {
        // base string
        let mut output = String::from("");
        // concatenate all output strings
        for plg in &plugin_list.items {
            output += &(plg.associated_fn)();
        }
        let xset_result = Command::new("xsetroot")
                          .arg("-name")
                          .arg(output.to_string())
                          .spawn();
        // xsetroot can fail if the system is hibernating/sleeping
        if let Err(_) = xset_result {
                writeln!(&setup_logger(), "xsetroot failed at {}", time())
                .expect("writing to log failed");
        };
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}
