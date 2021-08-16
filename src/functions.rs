//! Functions used in various rustatus plugins

use std::process::{Command, Stdio};
use chrono::Local;
use std::fs;
use ipgeolocate::{Locator, Service};

/// Returns a formatted string with the volume upon success.
/// Returns a formatted string with the character "U" if the
/// audio is unbalanced.
pub fn volume() -> String {
    let error = String::from(" V: E% | ");


    let status = Command::new("amixer")
    .arg("-D")
    .arg("pulse")
    .arg("get")
    .arg("Master")
    .stdout(Stdio::piped())
    .spawn();
    let amixer_result = match status {
        Ok(file) => match file.stdout {
                Some(file2) => file2,
                None => return error,
        },
        Err(_) => return error,
    };

    let awk_cmd = Command::new("awk")
    .arg("-F")
    .arg("[][]")
    .arg("{print $2}")
    .stdin(amixer_result)
    .output();
    
    let output = match awk_cmd {
        Ok(file) => file,
        Err(_) => return error,
    };

    let mut awk_output = String::from_utf8_lossy(&output.stdout)
                            .into_owned();

    // only retain the important printable info
    awk_output.retain(|c| !c.is_whitespace() && (c != '\n'));

    let split = awk_output.split("%");
    let vec: Vec<&str> = split.collect();

    if vec.len() >= 2 {
        if vec[0] == vec[1] {
            return String::from(format!(" V: {}% | ", vec[0]))
        }
        else {
            // linux updates one side faster when updating both
            // so ubalanced might be printed but it should be
            // corrected after a loop passes under normal circumstances
            return String::from(" V: U% | ")
        }
    }
    else {
        return error
    }
}

/// Returns the name of the currently connected network
/// inside a formatted string.
pub fn network_name() -> String{
    let output = Command::new("iwgetid")
    .arg("-r")
    .output();
    let output = match output {
        Ok(file) => file,
        Err(_) => return String::from(""),
    };
    let temp = String::from_utf8_lossy(&output.stdout).into_owned()
    .replace(|c: char| !c.is_ascii_alphanumeric(), "");

    return String::from(format!("N: {} | ", temp))
}

pub fn time() -> String {
    return Local::now().format("%I:%M %b-%d-%Y ").to_string();
}
pub fn mouse_battery() -> String {
    let contents = fs::read_to_string("/sys/class/power_supply/hidpp_battery_0/capacity_level");
    let output = match contents {
        Ok(file) => file,
        // backup battery location is not located at default spot
        Err(_) => match fs::read_to_string("/sys/class/power_supply/hidpp_battery_1/capacity_level"){
            Ok(file2) => file2,
            Err(_) => String::from("")
        },
    }
    .replace(|c: char| !c.is_ascii_alphanumeric(), "");

    let charging_status = fs::read_to_string("/sys/class/power_supply/hidpp_battery_0/status");
    match charging_status {
        Ok(file) => {
            if output == String::from("") {
                return String::from(format!("MB: {} | ", file)) 
            }
        },
        Err(_) => match fs::read_to_string("/sys/class/power_supply/hidpp_battery_1/status"){
            Ok(file2) => {
                if output == String::from("") {
                    return String::from(format!("MB: {} | ", file2))
                }
            },
            Err(_) => (),
        },
    };
    return String::from(format!("MB: {} | ", output))
}
pub fn internal_battery() -> String {
    let contents = fs::read_to_string("/sys/class/power_supply/BAT0/capacity");
    let output = match contents {
        Ok(file) => file,
        Err(_) => String::from(""),
    }
    .replace(|c: char| !c.is_ascii_alphanumeric(), "");

    let charging_status = fs::read_to_string("/sys/class/power_supply/BAT0/status");
    let indicator = 
    match charging_status {
            Ok(file) => 
                match file.replace(|c: char| !c.is_ascii_alphanumeric(), "").as_str() {
                        "Discharging" => "-" ,
                        _ => "+",
                    },
            Err(_) => "?"
        };
    return String::from(format!("B: {}%{} | ",output, indicator))
}

const DNS_RESOLVER: &str = "dig +short myip.opendns.com @resolver1.opendns.com";
pub fn ip() -> String {
    // free open source dns site. You can change it to your preferred one though
    let output = match Command::new(DNS_RESOLVER).output() {
        Ok(file) => file,
        Err(_) => return String::from(""),
    };
    let temp = String::from_utf8_lossy(&output.stdout).into_owned()
    .replace(|c: char| !c.is_ascii_alphanumeric(), "");

    return temp.to_string()
}

pub async fn location(ip: &str) -> (String, String) {
    let service = Service::IpApi;
    match Locator::get(ip, service).await {
        Ok(ip) => return (ip.city, ip.country),
        Err(_) => return (String::from("Undef City"),
                          String::from("Undef Country")),
    };
}

// not so much tests as instead quick ways to display output
// to run
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_test() {
        print!("\n{}\n", time());
    }
    #[test]
    fn sound_test() {
        print!("\n{}\n", volume());
    }
    #[test]
    fn wifi_test() {
        print!("\n{}\n", network_name());
    }
    #[test]
    fn mb_test() {
        print!("\n{}\n", mouse_battery());
    }
    // #[test]
    // fn location_test() {
    //     let locale = location(&ip());
    //     print!("\n{}\n", locale);
    // }
    #[test]
    fn ascii_test() {
        assert_eq!('#'.is_ascii_alphanumeric(), true)
    }
}