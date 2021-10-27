use regex::Regex;
use std::{fs::File, io::Read};

pub struct Settings {
    pub user: Option<String>,
    pub join_alert: bool,
    pub chat_reminders: bool,
    pub kick: bool,
    pub period: u32,
    pub directory: String,
}

impl Settings {
    pub fn new() -> Settings {
        let mut user: Option<String> = None;
        let mut chat_alerts: bool = true;
        let mut chat_reminders = true;
        let mut kick = true;
        let mut period: u32 = 15;
        let mut directory: String = String::from(".");

        let filename = "cfg/settings.cfg";

        let mut file = File::open(filename)
            .unwrap_or_else(|_| panic!("No settings file found in {}!", filename));
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|_| panic!("Failed to read file {} for settings.", filename));

        let regx = Regex::new(r#"^\s*(?P<setting>[\w\d]+)\s*=\s*(?P<value>.*)\s*$"#).unwrap();

        for line in contents.lines() {
            if let Some(caps) = regx.captures(line) {
                match &caps["setting"] {
                    "user" => {
                        user = get_uuid(&caps["value"]);
                        if user == None {
                            println!("No Userid set, this can be set in cfg/settings.cfg with user = [U:1:1234567]");
                        } else {
                            println!("Setting user id to {}", get_uuid(&caps["value"]).unwrap());
                        }
                    }
                    "join_alert" => {
                        if let Some(b) = to_bool(&caps["value"]) {
                            chat_alerts = b;
                            println!("Setting join alerts to {}", chat_alerts);
                        } else {
                            println!("Error reading value for setting chat_alerts");
                        }
                    }
                    "chat_reminders" => {
                        if let Some(b) = to_bool(&caps["value"]) {
                            chat_reminders = b;
                            println!("Setting chat reminders to {}", chat_reminders);
                        } else {
                            println!("Error reading value for setting chat_reminders");
                        }
                    }
                    "kick" => {
                        if let Some(b) = to_bool(&caps["value"]) {
                            kick = b;
                            println!("Setting kick to {}", kick);
                        } else {
                            println!("Error reading value for setting kick");
                        }
                    }
                    "period" => {
                        if let Ok(p) = &caps["value"].parse::<u32>() {
                            period = *p;
                            println!("Setting period to {} seconds", period);
                        }
                    }
                    "tf2_directory" => {
                        directory = caps["value"].to_string();
                    }
                    _ => {}
                }
            }
        }

        Settings {
            user,
            join_alert: chat_alerts,
            chat_reminders,
            kick,
            period,
            directory,
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
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
