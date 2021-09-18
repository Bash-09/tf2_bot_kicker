#![allow(dead_code)]

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
    pub new_players: Vec<String>,
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
            new_players: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
        self.new_players.clear();
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

    /// Call a votekick on any players detected as bots.
    /// If userid is set in cfg/settings.cfg then it will only attempt to call vote on bots in the same team
    /// There is no way of knowing if a vote is in progress or the user is on cooldown so votes will still be attempted
    pub fn kick_bots(&mut self) {
        if !self.settings.kick {
            return;
        }

        let mut bots: Vec<&Player> = Vec::new();

        for p in self.players.values().into_iter() {
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

    /// Print bots to console and send chat message in-game if necessary of current bots
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

        let mut invaders = false;
        let mut defenders = false;

        for p in bots.iter() {
            if p.team == Team::DEFENDERS {
                defenders = true;
            } else if p.team == Team::INVADERS {
                invaders = true;
            }
        }

        let mut alert: String = String::from("Bot alert! ");

        if invaders && defenders {
            alert.push_str("Both teams have BOTS: ");
        } else if let Some(userid) = &self.settings.user {
            if let Some(p) = self.players.get(userid) {
                if (p.team == Team::INVADERS && invaders) || (p.team == Team::DEFENDERS && defenders) {
                    alert.push_str("Our team has BOTS: ");
                } else {
                    alert.push_str("Enemy team has BOTS: ");
                }
            } else {
                alert.push_str("The server has BOTS: ");
            }
        } else {
            alert.push_str("The server has BOTS: ");
        }

        println!("Bots on server: ");
        for p in bots {
            alert.push_str(&format!("{} ", p.name));
            println!("{}", p);
        }

        if self.settings.chat_reminders {
            self.com.say(&alert);
        }
    }


    /// Update local info on server players
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

    /// Remove players who aren't present on the server anymore
    /// (This method will be called automatically in a rexes command)
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