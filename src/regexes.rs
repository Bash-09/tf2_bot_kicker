#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use std::{fs::OpenOptions, io::Write};

use crate::server::*;

use regex::{Captures, Regex};

use crate::server::player::*;

pub struct LogMatcher {
    pub r: Regex,
    pub f: fn(serv: &mut Server, str: &str, caps: Captures),
}

impl LogMatcher {
    pub fn new(r: Regex, f: fn(serv: &mut Server, str: &str, caps: Captures)) -> LogMatcher {
        LogMatcher { r, f }
    }
}

/*
    Useful commands:
        status
        tf_lobby_debug
        tf_party_debug //Not sure if this is actually useful, not really necessary

        callvote kick <userid>
        vote option<1/2> // Can't really use

*/

// Reads lines from output of the "status" command
// Includes players on server, player name, state, steamid, time connected
// If no player exists on the server with a steamid from here, it creates a new player and adds it to the list
pub const r_status: &str =
    r#"^#\s*(\d+)\s"(.*)"\s+\[(U:\d:\d+)\]\s+(\d*:?\d\d:\d\d)\s+\d+\s*\d+\s*(\w+).*$"#;
pub fn f_status(serv: &mut Server, str: &str, caps: Captures) {
    let steamid = caps[3].to_string();

    let mut state = State::Spawning;
    if caps[5].eq("active") {
        state = State::Active;
    }

    // Get connected time of player
    let time = get_time(caps[4].to_string());

    // Update an existing player
    if let Some(p) = serv.players.get_mut(&steamid) {
        p.time = time;
        p.state = state;
        p.accounted = true;

    // Create a new player entry
    } else {
        let name = caps[2].to_string();

        // Check if they are a bot according to the lists
        let mut bot = false;
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

            // Add suspected bot steamid and name to file
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("cfg/recorded_bots.txt")
                .expect("Failed to open/create cfg/recorded_bots.txt");

            if let Err(e) = write!(file, "\n[{}] - {}", &steamid, &name) {
                eprintln!("Couldn't write to cfg/recorded_bots.txt: {}", e);
            }
            serv.bot_checker.append_uuid(steamid.clone());
        }

        let mut new_connection: bool = false;
        if time < 20 {
            new_connection = true;
        }

        // Construct new player for the list
        let p = Player {
            userid: caps[1].to_string(),
            name,
            steamid,
            time,
            team: Team::None,
            state,
            bot,
            accounted: true,
            new_connection,
        };

        serv.players.insert(p.steamid.clone(), p);
    }
}

// Converts a given string time (e.g. 57:48 or 1:14:46) as an integer number of seconds
fn get_time(input: String) -> u32 {
    let mut t: u32 = 0;

    let splits: Vec<&str> = input.split(':').collect();
    let n = splits.len();

    for (i, v) in splits.iter().enumerate() {
        // let dt: u32 = v.parse::<u32>().expect(&format!("Had trouble parsing {} as u32", v));
        let dt: u32 = v
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Had trouble parsing {} as u32", v));
        t += 60u32.pow((n - i - 1) as u32) * dt;
    }

    t
}

// Reads lines from output of the "tf_lobby_debug" command
// Includes the team of players on the server
// NOTE: Teams are stored as INVADERS/DEFENDERS and does not swap when Red/Blu swaps so it cannot
// be used to reliably check which team the user is on, it can only check relative to the user (same/opposite team)
pub const r_lobby: &str =
    r#"^  Member\[(\d+)] \[(U:\d:\d+)]  team = TF_GC_TEAM_(\w+)  type = MATCH_PLAYER\s*$"#;
pub fn f_lobby(serv: &mut Server, str: &str, caps: Captures) {
    let mut team = Team::None;

    match &caps[3] {
        "INVADERS" => team = Team::Invaders,
        "DEFENDERS" => team = Team::Defenders,
        _ => {}
    }

    match serv.players.get_mut(&caps[2].to_string()) {
        None => {}
        Some(p) => {
            p.team = team;

            // Alert server of bot joining the server
            if p.new_connection && p.bot && serv.settings.join_alert {
                serv.new_bots.push((p.name.clone(), p.team));
                p.new_connection = false;
            }
        }
    }
}

pub const r_player_connect: &str = r#"^(.*) connected\s*$"#;
pub fn f_player_connect(serv: &mut Server, str: &str, caps: Captures) {
    // serv.new_players.push(String::from(&caps[1])); // Push name to list of new players

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
    serv.com
        .run_command("echo \"Commands: pause, resume, help, players, update\"", &serv.settings.key)
}

// Indicates all commands have been run server info updated and is ready to be cleared of old players
pub const r_refresh_complete: &str = r#"^refreshcomplete\s*$"#;
pub fn f_refresh_complete(serv: &mut Server, str: &str, caps: Captures) {
    serv.prune();
}

// Indicates old players have been removed and action can be taken against still-existing bots
pub const r_update: &str = r#"^prunecomplete\s*$"#;
pub fn f_update(serv: &mut Server, str: &str, caps: Captures) {
    serv.kick_bots();
    serv.announce_bots();
}

// Indicates the player is not currently in a casual lobby and to pause the program until they are
pub const r_inactive: &str = r#"^Failed to find lobby shared object\s*$"#;
pub fn f_inactive(serv: &mut Server, str: &str, caps: Captures) {
    println!("User is not connected to a valid server, pausing until a server is joined.");
    serv.set_inactive();
}
