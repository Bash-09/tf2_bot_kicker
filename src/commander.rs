use std::fs::{read_dir, File, OpenOptions};
use std::io::prelude::*;

// extern crate enigo;
// use enigo::{Enigo, Key, KeyboardControllable};

extern crate inputbot;
use inputbot::KeybdKey;

use crate::server::player::Player;

pub struct Commander {
    file: File,
    file_name: String,
    // key: Key,
    // keyboard: Enigo,
}

impl Commander {
    pub fn new(directory: &str) -> Commander {
        let dir: String = directory.to_string();

        if !check_directory(directory) {
            println!("Could not find tf2 directory in {}", directory);
            if !check_directory(".") {
                println!("Could not find tf2 directory in current folder. Please set a valid path in settings.cfg or run this program from the Team Fortress 2 folder.");
                std::process::exit(1);
            }
        }

        let file_name = format!("{}/tf/cfg/command.cfg", dir);

        Commander {
            file: create_command_file(&file_name),
            file_name,
            // key: Key::F7,
            // keyboard: Enigo::new(),
        }
    }

    /// Clears queued / recently run commands
    pub fn clear(&mut self) {
        match File::create(&self.file_name) {
            Err(_) => {
                eprintln!("Couldn't clear command file!");
            }
            Ok(file) => {
                self.file = file;
            }
        }
    }

    /// Pushes a new command to the queue
    pub fn push(&mut self, command: &str) {
        if self
            .file
            .write_all(format!("{}; ", command).as_bytes())
            .is_err()
        {
            eprintln!("Could not write command to command.cfg file!");
        }
    }

    /// Runs all queued commands
    pub fn run(&mut self) {
        // self.keyboard.key_click(self.key);
        KeybdKey::F7Key.press();
        KeybdKey::F7Key.release();

        // todo!("Reimplement keyboard input");
    }

    /// Clears queue and runs a command
    pub fn run_command(&mut self, command: &str) {
        self.clear();
        self.push(command);
        self.run();
    }

    pub fn say(&mut self, s: &str) {
        self.run_command(&format!("say \"{}\"", s));
    }

    pub fn kick(&mut self, p: &Player) {
        self.run_command(&format!("callvote kick {}", p.userid));
    }
}

fn create_command_file(file_name: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .unwrap()
}

fn check_directory(dir: &str) -> bool {
    //Check if valid TF2 directory
    read_dir(format!("{}/tf/cfg", dir)).is_ok()
}
