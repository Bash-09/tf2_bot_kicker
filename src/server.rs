use std::collections::HashMap;

pub mod player;
use player::Player;
use player::Team;

pub mod bot_checker;
use bot_checker::BotChecker;

use crate::commander::Commander;

pub struct Server {
    active: bool,
    pub players: HashMap<String, Player>,
    pub bot_checker: BotChecker,
    user: Option<String>,
}

impl Server {

    pub fn new() -> Server {
        Server{
            active: true,
            players: HashMap::new(),
            bot_checker: BotChecker::new(),
            user: None,
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

    pub fn add(&mut self, p: Player) {
        self.players.insert(p.uniqueid.clone(), p);
    }

    pub fn list_players(&self) {
        println!("Listing players:");
        for p in self.players.values() {
            println!("Player: {}", p);
        }
    }


    pub fn check_bots(&mut self, com: &mut Commander) {
        let mut bots: Vec<&Player> = Vec::new();
        let mut red: bool = false;
        let mut blu: bool = false;

        for p in self.players.values().into_iter() {
            if self.bot_checker.check_bot_name(&p.name) {
                bots.push(p);
                com.kick(p); //May need to disable till I find a better solution
                if p.team == Team::RED {
                    red = true;
                } else if p.team == Team::BLU {
                    blu = true
                }
            }
        }

        //self.list_players();

        if bots.is_empty() {return;}
        // Alert players of bots
        let mut alert: String = String::from("Bot alert! ");

        if red && blu {
            alert.push_str("Both teams have BOTS: ");
        } else if red {
            alert.push_str("RED Team has BOTS: ");
        } else if blu {
            alert.push_str("BLU Team has BOTS: ");
        }

        for p in bots.iter() {
            alert.push_str(&format!("{} ", p.name));
        }

        println!("{}", &alert);

        com.say(&alert);

            // Call vote kick


    }

}