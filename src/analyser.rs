use crate::commander::Commander;
use crate::timer::*;
use crate::{regexes::*, server::Server};
use regex::Regex;


/*
    Useful commands:
        status
        tf_lobby_debug

        callvote kick <userid>
        vote option<1/2>

*/

pub struct Analyser {
    t: Timer,
    com: Commander,
    serv: Server,
    reg: Vec<LogMatcher>,
}

impl Analyser {

    pub fn new(com: Commander) -> Analyser {

        let mut reg: Vec<LogMatcher> = Vec::new();

        // reg.push(LogMatcher::new(Regex::new(reg_status), status));
        reg.push(LogMatcher::new(
            Regex::new(r_status).unwrap(),
            f_status
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_lobby).unwrap(),
            f_lobby
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_player_connect).unwrap(),
            f_player_connect
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_user_connect).unwrap(),
            f_user_connect
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_user_disconnect).unwrap(),
            f_user_disconnect
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_list_players).unwrap(),
            f_list_players
        ));
        // reg.push(LogMatcher::new(
        //     Regex::new(r_update_players).unwrap(),
        //     f_update_players
        // ));
        reg.push(LogMatcher::new(
            Regex::new(r_pause).unwrap(),
            f_pause
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_resume).unwrap(),
            f_resume
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_help).unwrap(),
            f_help
        ));
        reg.push(LogMatcher::new(
            Regex::new(r_update).unwrap(),
            f_update
        ));


        Analyser {
            t: Timer::new(),
            com,
            serv: Server::new(),
            reg,
        }

    }


    pub fn update(&mut self, string: &str) {

        //Run regex matches
        for r in self.reg.iter() {
            match r.r.captures(string) {
                None => {},
                Some(c) => {
                    (r.f)(&mut self.serv, &mut self.com, string, c);
                }
            }
        }

        //Update interval stuff
        if !self.serv.is_active() {
            return;
        }

        if !self.t.go() {
            return;
        }

        // Refresh server
        if self.t.intervals() % self.serv.settings.period == 0 {
            self.com.update_info(&mut self.serv);
        }

    }

}
