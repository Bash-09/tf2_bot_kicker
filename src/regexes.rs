#![allow(non_upper_case_globals)]
#![allow(unused_variables)]


use crate::server::*;

use regex::{Captures, Regex};

use crate::server::player::*;

pub struct LogMatcher {
    pub r: Regex,
    pub f: fn(serv: &mut Server, str: &str, caps: Captures),
}

impl LogMatcher {
    pub fn new(r: Regex, f: fn(serv: &mut Server, str: &str, caps: Captures)) -> LogMatcher {
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
pub fn f_status(serv: &mut Server, str: &str, caps: Captures) {
    let mut state = State::Spawning;
    if caps[5].eq("active") {
        state = State::Active;
    }
    let mut bot = false;

    let name = caps[2].to_string();
    let steamid = caps[3].to_string();

    if serv.bot_checker.check_bot_steamid(&steamid) {
        bot = true;

        if !serv.players.contains_key(&steamid) {
            println!("Known Bot joining:   {}", name);
        }
    } else if serv.bot_checker.check_bot_name(&name) {
        bot = true;

        if !serv.players.contains_key(&steamid) {
            println!("Unknown bot joining: {} - [{}]", name, steamid);
        }
    }

    let p = Player {
        userid: caps[1].to_string(),
        name,
        steamid,
        time: 0, // Not implemented
        team: Team::NONE,
        state,
        bot,
        accounted: true,
    };

    serv.players.insert(p.steamid.clone(), p);

}

pub const r_lobby: &str = r#"^  Member\[(\d+)] \[(U:\d:\d+)]  team = TF_GC_TEAM_(\w+)  type = MATCH_PLAYER\s*$"#;
pub fn f_lobby(serv: &mut Server, str: &str, caps: Captures) {
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

pub const r_player_connect: &str = r#"^(.*) connected\s*$"#;
pub fn f_player_connect(serv: &mut Server, str: &str, caps: Captures) {
    if serv.bot_checker.check_bot_name(&caps[1]) {
        println!("Bot has joined: {}", &caps[1]);

        serv.kick_bots();
    }
}

pub const r_user_connect: &str = r#"^Connected to .*"#;
pub fn f_user_connect(serv: &mut Server, str: &str, caps: Captures) {
    println!("Connected to server.");
    serv.set_active();
}

pub const r_user_disconnect: &str = r#"^Disconnecting from .*"#;
pub fn f_user_disconnect(serv: &mut Server, str: &str, caps: Captures) {
    println!("Disconnected from server.");
    serv.set_inactive();
    serv.clear();
}


pub const r_list_players: &str = r#"^players\s*$"#;
pub fn f_list_players(serv: &mut Server, str: &str, caps: Captures) {
    serv.list_players();
}

pub const r_pause: &str = r#"^pause\s*$"#;
pub fn f_pause(serv: &mut Server, str: &str, caps: Captures) {
    println!("Pausing.");
    serv.set_inactive();
}

pub const r_resume: &str = r#"^resume\s*$"#;
pub fn f_resume(serv: &mut Server, str: &str, caps: Captures) {
    println!("Resuming.");
    serv.set_active();
    serv.refresh();
}

pub const r_help: &str = r#"^help\s*$"#;
pub fn f_help(serv: &mut Server, str: &str, caps: Captures) {
    println!("Commands: \n pause\nresume\nhelp\nplayers\nupdate");
    serv.com.run_command("echo \"Commands: pause, resume, help, players, update\"")
}

pub const r_refresh_complete: &str = r#"^refreshcomplete\s*$"#;
pub fn f_refresh_complete(serv: &mut Server, str: &str, caps: Captures) {
    serv.prune();
}

pub const r_update: &str = r#"^prunecomplete\s*$"#;
pub fn f_update(serv: &mut Server, str: &str, caps: Captures) {
    serv.kick_bots();
    serv.announce_bots();
}

pub const r_inactive: &str = r#"^Failed to find lobby shared object\s*$"#;
pub fn f_inactive(serv: &mut Server, str: &str, caps: Captures) {
    println!("User is not connected to a valid server, pausing until a server is joined.");
    serv.set_inactive();
}