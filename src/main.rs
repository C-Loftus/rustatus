use std::process::Command;
mod lib;
use lib::*;
use std::thread;
use std::time::Duration;

fn main() {

    loop {
        let volume = awk_volume();
        let wifi = wifi_name();
        let time = time();
        let mb = mouse_bat();
        let bat = internal_bat();

        let output = volume + &wifi + &mb + &bat + &time;
        // print!("{}", output);
        // bat, mouse bat, usb conn, ip, brightness
        Command::new("xsetroot")
        .arg("-name")
        .arg(output.to_string())
        .spawn()
        .expect("xsetroot command failed to start");
        thread::sleep(std::time::Duration::from_millis(100));
        }
    }
