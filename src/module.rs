
use super::functions::*;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub struct ModuleList {
    items: Vec<&'static Module>,
} 

pub struct Module {
    pub get_data: Box<dyn Fn()>,
    pub rate: u8,
}

enum FuncWrapper {
    New(&'static dyn Fn() -> String), 
}

// hash map literal with fn pointers.
// rust doesn't have generic hashmap literals
// let text_to_funct = HashMap<&str, FuncWrapper> = vec![ ("internal battery", FuncWrapper::New(&internal_battery)), 
//                                                     // ("mouse battery", &mouse_battery),
//                                                     // ("volume", &volume),
//                                                     // ("network name", &network_name),
//                                                     // ("time", &time)
//                                                 ].into_iter().collect();

impl Module {
    fn module_map() -> () {
        // hashmap doesn't support &dyn Fn with different fn
        // so it needs to be wrapped
        let map = hashmap!{
            "internal battery" => FuncWrapper::New(&internal_battery),
            "mouse battery" => FuncWrapper::New(&mouse_battery),
            "volume" => FuncWrapper::New(&volume),
            "network name" => FuncWrapper::New(&network_name), 
            "time" => FuncWrapper::New(&time),
            };
    }
}


// hash map

const CONFIG_PATH: &str = "modules.conf";
impl ModuleList {
    fn new() -> ModuleList {

        let filename = CONFIG_PATH;
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
    
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            // Show the line and its number.
            println!("{}. {}", index + 1, line);
        }

        // ModuleList {

        // }
    }

    fn add_module (&mut self, m: &'static Module) -> () {
        self.items.push(m);
        }
    

    }
