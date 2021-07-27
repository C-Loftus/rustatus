use std::process::Command;
mod lib;
use lib::*;

fn main() {

    let volume = awk_volume();
    let wifi = wifi_name();
    let time = time();
    let mb = mouse_bat();

    let output = volume + &wifi + &mb + &time;
    print!("{}", output);
    // bat, mouse bat, usb conn, ip, brightness
    Command::new("xsetroot")
    .arg("-name")
    .arg(output.to_string())
    .spawn()
    .expect("ls command failed to start");

    }
