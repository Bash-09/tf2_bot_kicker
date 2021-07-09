use std::{fs::{File, OpenOptions}};
use std::io::prelude::*;

extern crate enigo;
use enigo::{Enigo, KeyboardControllable, Key};

use crate::server::{Server, player::Player};

pub struct Commander {
    file: File,
    file_name: String,
    key: Key,
    keyboard: Enigo,
}

pub const COM_STATUS: &str = "status";
pub const COM_LOBBY: &str = "tf_lobby_debug";

impl Commander {

    pub fn new(file_name: String) -> Commander {


        let com = Commander {
            file: create_command_file(&file_name),
            file_name,
            key: Key::F7,
            keyboard: Enigo::new(),
        };

        return com;
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
        if let Err(_) = self.file.write_all(format!("{}; ", command).as_bytes()) {
            eprintln!("Could not write command to command.cfg file!");
        }
    }

    /// Runs all queued commands
    pub fn run(&mut self) {
        self.keyboard.key_click(self.key);
    }

    /// Clears queue and runs a command
    pub fn run_command(&mut self, command: &str) {
        //println!("Running \"{}\"", command);
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


    pub fn update_info(&mut self, serv: &mut Server) {
        println!("Refreshing server info.");
        serv.clear();
        self.clear();
        self.push(COM_STATUS);
        self.push("wait 200");
        self.push(COM_LOBBY);
        self.push("wait 100");
        self.push("echo updatecomplete");
        self.run();
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