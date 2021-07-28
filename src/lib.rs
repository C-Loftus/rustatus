use std::process::{Command, Stdio};
use chrono::Local;
use std::fs;
use ipgeolocate::{Locator, Service};

/********************* */
// Options aren't returned by these functions. Instead error handling is
// just done within the function itself, i.e. by returning a blank string
// or a string that is formatted but just has no number

// It is easier to handle each case in the functions since each one is
// different in its format. Updating the String to a long error is rarely 
// desirable,since it will cause the hostname to become much longer and thus 
// make the status bar unpleasing to the eye.

/********************* */

pub fn awk_volume() -> String {
    let status = Command::new("amixer")
    .arg("-D")
    .arg("pulse")
    .arg("get")
    .arg("Master")
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    let output = Command::new("awk")
    .arg("-F")
    .arg("[][]")
    .arg("{print $2}")
    .stdin(status.stdout.unwrap())
    .output();
    let output = match output {
        Ok(file) => file,
        Err(_) => return String::from(""),
    };

    let mut awk_output = String::from_utf8_lossy(&output.stdout)
                            .into_owned();
 
    awk_output.retain(|c| !c.is_whitespace() && (c != '\n'));
    let split = awk_output.split("%");
    let vec: Vec<&str> = split.collect();

    if vec.len() >= 2 {
        if vec[0] == vec[1] {
            return String::from(format!(" V: {}% | ", vec[0]))
        }
        else {
            // linux updates one side faster when updating both, 
            //unbalance is always corrected after a second passes
            return String::from(" V: U% | ")
        }
    }
    else {
        return String::from(" V: E% | ")
    }
}

pub fn wifi_name() -> String{
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
pub fn mouse_bat() -> String {
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
pub fn internal_bat() -> String {
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


pub fn get_ip() -> String {
    let output = Command::new("dig +short myip.opendns.com @resolver1.opendns.com")
    .output();
    let output = match output {
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
// TODO turn into proper tests. 

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
        print!("\n{}\n", awk_volume());
    }
    #[test]
    fn wifi_test() {
        print!("\n{}\n", wifi_name());
    }
    #[test]
    fn mb_test() {
        print!("\n{}\n", mouse_bat());
    }
    #[test]
    fn location_test() {
        let locale = location(&get_ip());
        print!("\n{}\n", locale);
    }
}