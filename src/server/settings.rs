use inputbot::KeybdKey;
use regex::Regex;
use std::{fs::File, io::Read};

pub struct Settings {
    pub user: Option<String>,
    pub join_alert: bool,
    pub chat_reminders: bool,
    pub kick: bool,
    pub period: u32,
    pub directory: String,
    pub key: KeybdKey,
}

impl Settings {
    pub fn new() -> Settings {
        let mut user: Option<String> = None;
        let mut chat_alerts: bool = true;
        let mut chat_reminders = true;
        let mut kick = true;
        let mut period: u32 = 15;
        let mut directory: String = String::from(".");
        let mut key = KeybdKey::F7Key;

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
                    "key" => {
                        key = get_key(&caps["value"]);
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
            key,
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

fn get_key(str: &str) -> KeybdKey {
    match str.to_ascii_lowercase().trim() {
        "backspace" => KeybdKey::BackspaceKey,
        "tab" => KeybdKey::TabKey,
        "enter" => KeybdKey::EnterKey,
        "escape" => KeybdKey::EscapeKey,
        "space" => KeybdKey::SpaceKey,
        "home" => KeybdKey::HomeKey,
        "left" | "leftarrow" => KeybdKey::LeftKey,
        "up" | "uparrow" => KeybdKey::UpKey,
        "right" | "rightarrow" => KeybdKey::RightKey,
        "down" | "downarrow" => KeybdKey::DownKey,
        "ins" | "insert" => KeybdKey::InsertKey,
        "del" | "delete" => KeybdKey::DeleteKey,
        "np0" | "numpad0" | "kp_ins" => KeybdKey::Numpad0Key,
        "np1" | "numpad1" | "kp_end" => KeybdKey::Numpad1Key,
        "np2" | "numpad2" | "kp_downarrow" => KeybdKey::Numpad2Key,
        "np3" | "numpad3" | "kp_pgdn" => KeybdKey::Numpad3Key,
        "np4" | "numpad4" | "kp_leftarrow" => KeybdKey::Numpad4Key,
        "np5" | "numpad5" | "kp_5" => KeybdKey::Numpad5Key,
        "np6" | "numpad6" | "kp_rightarrow" => KeybdKey::Numpad6Key,
        "np7" | "numpad7" | "kp_home" => KeybdKey::Numpad7Key,
        "np8" | "numpad8" | "kp_uparrow" => KeybdKey::Numpad8Key,
        "np9" | "numpad9" | "kp_pgup" => KeybdKey::Numpad9Key,
        "a" => KeybdKey::AKey,
        "b" => KeybdKey::BKey,
        "c" => KeybdKey::CKey,
        "d" => KeybdKey::DKey,
        "e" => KeybdKey::EKey,
        "f" => KeybdKey::FKey,
        "g" => KeybdKey::GKey,
        "h" => KeybdKey::HKey,
        "i" => KeybdKey::IKey,
        "j" => KeybdKey::JKey,
        "k" => KeybdKey::KKey,
        "l" => KeybdKey::LKey,
        "m" => KeybdKey::MKey,
        "n" => KeybdKey::NKey,
        "o" => KeybdKey::OKey,
        "p" => KeybdKey::PKey,
        "q" => KeybdKey::QKey,
        "r" => KeybdKey::RKey,
        "s" => KeybdKey::SKey,
        "t" => KeybdKey::TKey,
        "u" => KeybdKey::UKey,
        "v" => KeybdKey::VKey,
        "w" => KeybdKey::WKey,
        "x" => KeybdKey::XKey,
        "y" => KeybdKey::YKey,
        "z" => KeybdKey::ZKey,
        "f1" => KeybdKey::F1Key,
        "f2" => KeybdKey::F2Key,
        "f3" => KeybdKey::F3Key,
        "f4" => KeybdKey::F4Key,
        "f5" => KeybdKey::F5Key,
        "f6" => KeybdKey::F6Key,
        "f7" => KeybdKey::F7Key,
        "f8" => KeybdKey::F8Key,
        "f9" => KeybdKey::F9Key,
        "f10" => KeybdKey::F10Key,
        "f11" => KeybdKey::F11Key,
        "f12" => KeybdKey::F12Key,
        "numlock" => KeybdKey::NumLockKey,
        "scrolllock" => KeybdKey::ScrollLockKey,
        "capslock" | "caps" => KeybdKey::CapsLockKey,
        "shift" | "lshift" => KeybdKey::LShiftKey,
        _ => KeybdKey::F7Key
    }
}