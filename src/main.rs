
/*

    Add "-condebug" to TF2 launch options to write console to log file
    (optional) Add "-conclearlog" to clear console.log file on game statup
        (If set, this program must be started after TF2, if not then doesn't matter)
        (If not set, the log file will keep growing and may need manual deleting after a while)



*/

mod logwatcher;

use logwatcher::*;
use std::fs::*;

mod commander;
use commander::{Commander};

mod analyser;
use analyser::{Analyser};

mod regexes;
mod server;

mod timer;

fn main() {

    let print_console = false;


    //Get TF2 directory
    let mut dir: &str = "";

    let dirs = vec![
        "/Program Files (x86)/Steam/Steamapps/Common/Team Fortress 2",
        "$HOME/.steam/Steam/Steamapps/Common/Team Fortress 2",
        "."
    ];

    let mut found_dir = false;

    for d in dirs.iter() {
        if check_directory(d) {
            dir = d;
            println!("Found TF2 directory");
            found_dir = true;
            break;
        }
    }

    if !found_dir {
        println!("Couldn't find TF2 directory, try running this program again directly in the Team Fortress 2 folder.");
        std::process::exit(1);
    }

    // Setup commander and analyser
    let exec_file = format!("{}/tf/cfg/command.cfg", dir);

    let com = Commander::new(exec_file);
    let mut analyser = Analyser::new(com);

    // Setup watcher on log file
    let log_file = format!("{}/tf/console.log", dir);
    let mut lw = LogWatcher::register(log_file).unwrap();

    println!("Setup complete, happy gaming!");

    lw.watch(&mut move |line: String| {
        if print_console {
            println!("Console: {}", line);
        }

        analyser.update(&line);
        LogWatcherAction::None
    });

}


fn check_directory(dir: &str) -> bool {
    //Attempt to find tf directory
    match read_dir(format!("{}/tf/cfg", dir)) {
        Ok(_) => {return true},
        Err(_)=> {
            println!("tf not in {}", dir);
            return false;
        }
    }
}


// Make sure player has a key bound to execute the custom command config
fn _bind_command_key() {
    // Not implemented

    //Please add a line to you autoexec.cfg:
    //bind F7 "exec command.cfg"
}