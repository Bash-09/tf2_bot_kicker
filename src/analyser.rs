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
    pub serv: Server,
    reg: Vec<LogMatcher>,
}

impl Analyser {
    pub fn new() -> Analyser {
        // let mut reg: Vec<LogMatcher> = Vec::new();
        let reg = vec![
            LogMatcher::new(Regex::new(r_status).unwrap(), f_status),
            LogMatcher::new(Regex::new(r_lobby).unwrap(), f_lobby),
            LogMatcher::new(Regex::new(r_player_connect).unwrap(), f_player_connect),
            LogMatcher::new(Regex::new(r_user_connect).unwrap(), f_user_connect),
            LogMatcher::new(Regex::new(r_user_disconnect).unwrap(), f_user_disconnect),
            LogMatcher::new(Regex::new(r_list_players).unwrap(), f_list_players),
            LogMatcher::new(Regex::new(r_pause).unwrap(), f_pause),
            LogMatcher::new(Regex::new(r_resume).unwrap(), f_resume),
            LogMatcher::new(Regex::new(r_help).unwrap(), f_help),
            LogMatcher::new(Regex::new(r_update).unwrap(), f_update),
            LogMatcher::new(Regex::new(r_inactive).unwrap(), f_inactive),
            LogMatcher::new(Regex::new(r_refresh_complete).unwrap(), f_refresh_complete),
        ];

        Analyser {
            t: Timer::new(),
            serv: Server::new(),
            reg,
        }
    }

    pub fn update(&mut self, string: &str) {
        //Run regex matches
        for r in self.reg.iter() {
            match r.r.captures(string) {
                None => {}
                Some(c) => {
                    (r.f)(&mut self.serv, string, c);
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
        if self.t.intervals() % self.serv.settings.period == 0 && !self.t.done {
            self.serv.refresh();
            self.t.done = true;
        }
    }
}
