use std::io::{BufRead, BufReader, Read};
use std::fs::File;

use regex::Regex;

pub struct BotChecker {
    bots_reg: Vec<Regex>,
    bots_str: Vec<String>,


}

impl BotChecker {

    pub fn new() -> BotChecker {
        let filename = "bots.txt";

        let mut file = File::open(filename).expect(&format!("No file named {} for bot name regexes!", filename));
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).expect(&format!("Failed to read file {} for bot regexes.", filename));

        let mut regs: Vec<Regex> = Vec::new();
        let mut str: Vec<String> = Vec::new();

        for line in contents.lines() {
            match Regex::new(line) {
                Ok(r) => {regs.push(r);}
                Err(_)      => {eprintln!("Failed to compile regex for name: {}", line);}
            }
        }

        BotChecker {
            bots_reg: regs,
            bots_str: str
        }
    }


    pub fn check_bot_name(&self, name: &str) -> bool {

        for b in self.bots_reg.iter() {
            match b.captures(name) {
                Some(_) => {return true;}
                None    => {}
            }
        }
        for b in self.bots_str.iter() {
            if b.eq(name) {
                return true;
            }
        }

        false
    }

}
