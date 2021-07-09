#![allow(non_upper_case_globals)]
#![allow(unused_variables)]


use crate::commander::Commander;
use crate::server::*;

use regex::{Captures, Regex};

use crate::server::player::*;

pub struct LogMatcher {
    pub r: Regex,
    pub f: fn(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures),
}

impl LogMatcher {
    pub fn new(r: Regex, f: fn(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures)) -> LogMatcher {
        LogMatcher {
            r,
            f,
        }
    }
}

/*
    Useful commands:
        status
        tf_lobby_debug
        tf_party_debug

        callvote kick <userid>
        vote option<1/2>

*/

pub const r_status: &str = r#"^#\s*(\d+)\s"(.*)"\s+\[(U:\d:\d+)\]\s+(\d*:?\d\d:\d\d)\s+\d+\s*\d+\s*(\w+).*$"#;
pub fn f_status(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    let p = Player {
        userid: caps[1].to_string(),
        name:   caps[2].to_string(),
        uniqueid: caps[3].to_string(),
        time: 0, // Add time conversion
        team: Team::NONE,
    };

    // println!("{}", p);
    serv.add(p);

}

pub const r_lobby: &str = r#"^  Member\[(\d+)] \[(U:\d:\d+)]  team = TF_GC_TEAM_(\w+)  type = MATCH_PLAYER\s*$"#;
pub fn f_lobby(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    let mut team = Team::NONE;

    match &caps[3] {
        "INVADERS" => {team = Team::BLU},
        "DEFENDERS" => {team = Team::RED},
        _ => {},
    }

    match serv.players.get_mut(&caps[2].to_string()) {
        None => {},
        Some(p) => {
            p.team = team;
        }
    }

}

/// Player connecting - announces to chat if name matches that of a bot
pub const r_player_connect: &str = r#"^(.*) connected\s*$"#;
pub fn f_player_connect(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    if serv.bot_checker.check_bot_name(&caps[1]) {
        println!("Bot {} detected joining.", &caps[1]);
        com.say(&format!("Bot Alert! {} is joining the game.", &caps[1]));
    }
}

pub const r_user_connect: &str = r#"^Connected to .*"#;
pub fn f_user_connect(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    println!("Connected to server.");
    serv.set_active();
}

pub const r_user_disconnect: &str = r#"^Disconnecting from .*"#;
pub fn f_user_disconnect(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    println!("Disconnected from server.");
}


pub const r_list_players: &str = r#"^players\s*$"#;
pub fn f_list_players(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    serv.list_players();
}

pub const r_update_players: &str = r#"^update\s*$"#;
pub fn f_update_players(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    println!("Updating server info due to: {}", str);
    com.update_info(serv);
}

pub const r_pause: &str = r#"^pause\s*$"#;
pub fn f_pause(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    println!("Pausing.");
    serv.set_inactive();
}

pub const r_resume: &str = r#"^resume\s*$"#;
pub fn f_resume(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    println!("Resuming.");
    serv.set_active();
}

pub const r_help: &str = r#"^help\s*$"#;
pub fn f_help(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    println!("Commands: \n pause\nresume\nhelp\nplayers\nupdate");
    com.run_command("echo \"Commands: pause, resume, help, players, update\"")
}

pub const r_update: &str = r#"^updatecomplete\s*$"#;
pub fn f_update(serv: &mut Server, com: &mut Commander, str: &str, caps: Captures) {
    serv.check_bots(com);
}