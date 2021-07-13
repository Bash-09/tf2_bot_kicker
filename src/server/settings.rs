use std::{fs::File, io::Read};
use regex::Regex;

pub struct Settings {
    pub user: Option<String>,
    pub chat_alerts: bool,
    pub kick: bool,
    pub period: u32,
}

impl Settings {
    
    pub fn new() -> Settings {
        let mut user: Option<String> = None;
        let mut chat_alerts = true;
        let mut kick = true;
        let mut period: u32 = 15;

        let filename = "cfg/settings.cfg";

        let mut file = File::open(filename).expect(&format!("No settings file found in {}!", filename));
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).expect(&format!("Failed to read file {} for settings.", filename));

        let regx = Regex::new(r#"^\s*(?P<setting>[\w\d]+)\s*=\s*(?P<value>.*)\s*$"#).unwrap();

        for line in contents.lines() {
            if let Some(caps) = regx.captures(line) {
                match &caps["setting"] {
                    "user"          => {
                        user = get_uuid(&caps["value"]);
                        println!("Setting user id to {}", get_uuid(&caps["value"]).unwrap());
                    },
                    "chat_alerts"   => {
                        if let Some(b) = to_bool(&caps["value"]) {
                            chat_alerts = b;
                            println!("Setting chat alerts to {}", chat_alerts);
                        } else {
                            println!("Error reading value for setting chat_alerts");
                        }
                    },
                    "kick"          => {
                        if let Some(b) = to_bool(&caps["value"]) {
                            kick = b;
                            println!("Setting kick to {}", kick);
                        } else {
                            println!("Error reading value for setting kick");
                        }
                    },
                    "period"        => {
                        if let Ok(p) = &caps["value"].parse::<u32>() {
                            period = *p;
                            println!("Setting period to {} seconds", period);
                        }
                    }
                    _ => {}
                }
            }
        }


        Settings {
            user,
            chat_alerts,
            kick,
            period,
        }
    }


}

fn get_uuid(s: &str) -> Option<String> {
    if let Some(caps) = Regex::new(r#"\[?(U:\d:\d+)\]?"#).unwrap().captures(s) {
        return Some(caps[1].to_string());
    } else {
        println!("Error reading user uuid");
    }
    None
}

fn to_bool(s: &str) -> Option<bool> {
    match s {
        "true"  => Some(true),
        "false" => Some(false),
        _       => None,
    }
}