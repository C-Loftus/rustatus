pub mod plugins;
pub mod functions;
pub mod logging;

use functions::*;
use logging::*;

use std::process::Command;
use std::thread;
use gag::Redirect;
use std::io::prelude::*;

#[macro_use]
extern crate lazy_static;
#[macro_use] 
extern crate maplit;



fn main() {
    /***************************************/
    // add/edit all your desired plugins, in order
    let plugins: Vec<&dyn Fn() -> String> = 
        vec![
            &volume,
            &network_name, 
            &mouse_battery,
            &internal_battery,
            &time,
        //  &example_func,
        ];
    /***************************************/

    // panics are logged to home dir
    let log = setup_logger();
    let _stderr_redirect = Redirect::stderr(log).unwrap();

    loop {
        // base string
        let mut output = String::from("");
        // concatenate all output strings
        for func in &plugins {
            output += &func();
        }
        let xset_result = Command::new("xsetroot")
        .arg("-name")
        .arg(output.to_string())
        .spawn();
        // xsetroot can fail if the system is hibernating/sleeping
        let xprocess = match xset_result {
            Ok(result) => Some(result),
            Err(_) => {
                writeln!(&setup_logger(), "xsetroot failed at {}", time())
                .expect("writing to log failed");
                None
            }
        };
        thread::sleep(std::time::Duration::from_millis(1000));
        // only cleanup after sleeping. Gives time for os
        // match xprocess {
        //     Some(mut res) => {
        //         // kill is also an option
        //         let _ = res.wait();
        //     },
        //     // no process to kill/wait for
        //     None => (),
        // }
    }
}
