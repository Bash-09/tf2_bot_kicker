use std::collections::HashMap;

pub mod player;
use player::Player;
use player::Team;

pub mod bot_checker;
use bot_checker::BotChecker;

use crate::commander::Commander;
use crate::server::player::State;

mod settings;
use settings::Settings;


pub const COM_STATUS: &str = "status";
pub const COM_LOBBY: &str = "tf_lobby_debug";

pub struct Server {
    active: bool,
    pub players: HashMap<String, Player>,
    pub settings: Settings,
    pub com: Commander,
    pub bot_checker: BotChecker,
}

impl Server {

    pub fn new() -> Server {
        let settings = Settings::new();
        let com = Commander::new(&settings.directory);

        Server{
            active: true,
            players: HashMap::with_capacity(24),
            settings,
            com,
            bot_checker: BotChecker::new(),
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
    }

    pub fn set_active(&mut self) {
        self.active = true;
        self.players.clear();
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn list_players(&self) {
        println!("Listing players:");
        for p in self.players.values() {
            println!("Player: {}", p);
        }
    }


    pub fn get_bots(&self) -> Vec<&Player> {

        let mut bots: Vec<&Player> = Vec::new();

        for p in self.players.values().into_iter() {
            if p.bot {
                bots.push(p);
            }
        }

        bots
    }

    pub fn kick_bots(&mut self) {
        if !self.settings.kick {
            return;
        }

        let mut bots: Vec<&Player> = Vec::new();

        for p in self.players.values().into_iter() {
            //println!("{}", p);
            if p.bot {
                bots.push(p);
            }
        }
        bots = bots.into_iter().filter(|p| {
            p.state == State::Active && p.accounted
        }).collect();

        for p in bots {
            match &self.settings.user {
                None => {
                    println!("Calling votekick.");
                    self.com.run_command("echo calling votekick");
                    println!("{}", p);
                    self.com.kick(p);
                }
                Some(id) => {
                    if p.team == self.players.get(id).unwrap().team {
                        self.com.kick(p);
                    }
                }
            }
        }
    }


    pub fn announce_bots(&mut self) {
        let mut bots: Vec<&Player> = Vec::new();
        for p in self.players.values().into_iter() {
            if p.bot {
                bots.push(p);
            }
        }
        bots = bots.into_iter().filter(|p| {
            p.state == State::Active && p.accounted
        }).collect();

        if bots.is_empty() {
            return;
        }

        let mut red = false;
        let mut blu = false;

        for p in bots.iter() {
            if p.team == Team::RED {
                red = true;
            } else if p.team == Team::BLU {
                blu = true;
            }
        }

        let mut alert: String = String::from("Bot alert! ");

        if red && blu {
            alert.push_str("Both teams have BOTS: ");
        } else if red {
            alert.push_str("RED Team has BOTS: ");
        } else if blu {
            alert.push_str("BLU Team has BOTS: ");
        } else {
            alert.push_str("There are bots: ");
        }

        println!("Bots on server: ");
        for p in bots.iter() {
            alert.push_str(&format!("{} ", p.name));
            println!("{}", p);
        }

        if self.settings.chat_alerts {
            self.com.say(&alert);
        }
    }


    pub fn refresh(&mut self) {
        println!("Refreshing server.");

        for p in self.players.values_mut().into_iter() {
            p.accounted = false;
        }

        self.com.clear();
        self.com.push(COM_STATUS);
        self.com.push("wait 200");
        self.com.push(COM_LOBBY);
        self.com.push("wait 100");
        self.com.push("echo refreshcomplete");
        self.com.run();

    }

    pub fn prune(&mut self) {
        self.players.retain(|_, v| {
            if !v.accounted && v.bot {
                println!("Bot disconnected: {}", v.name);
            }
            v.accounted
        });

        self.com.run_command("wait 100; echo prunecomplete");

    }

}