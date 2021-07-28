mod lib;
use lib::*;
use std::process::Command;
use std::thread;

fn main() {
    // all your desired modules, in order
    let modules: Vec<&dyn Fn() -> String> = 
        vec![
            &awk_volume,
            &wifi_name, 
            &mouse_bat,
            &internal_bat,
            &time,
        ];


    loop {
        // base string
        let mut output = String::from("");
        // concatenate all output strings
        for func in &modules {
            output += &func();
        }

        print!("\n{}", output);

        Command::new("xsetroot")
        .arg("-name")
        .arg(output.to_string())
        .spawn()
        .expect("xsetroot command failed to start");
        thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
