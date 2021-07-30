mod lib;
use lib::*;
use std::process::Command;
use std::thread;
use std::fs::OpenOptions;
use gag::Redirect;
use std::io::prelude::*;

fn main() {
    /***************************************/
    // add/edit all your desired modules, in order
    let modules: Vec<&dyn Fn() -> String> = 
        vec![
            &awk_volume,
            &wifi_name, 
            &mouse_bat,
            &internal_bat,
            &time,
        //  &example_func,
        ];
    /***************************************/

    // panics are logged to home dir
    let log = OpenOptions::new()
    .truncate(true)
    .read(true)
    .create(true)
    .write(true)
    .open(dirs::home_dir().unwrap().join("rustatus"))
    .unwrap();
    let _stderr_redirect = Redirect::stderr(log).unwrap();

    loop {
        // base string
        let mut output = String::from("");
        // concatenate all output strings
        for func in &modules {
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
                // have to recreate variable here since
                // stderr_redirect takes ownership and I need 
                // another copy. Just remaking the variable is
                // easiest and isn't a performance issue since 
                // it occurs rarely
                let log = OpenOptions::new()
                .truncate(true)
                .read(true)
                .create(true)
                .write(true)
                .open(dirs::home_dir().unwrap().join("rustatus"))
                .unwrap();
                writeln!(&log, "xsetroot failed at {}", time())
                .expect("writing to log failed");
                None
            }
        };
        thread::sleep(std::time::Duration::from_millis(1000));
        // only cleanup after sleeping. Gives time for os
        match xprocess {
            Some(mut res) => {
                // kill is also an option
                let _ = res.wait();
            },
            // no process to kill/wait for
            None => (),
        }
    }
}
