
/*

    Add "-condebug" to TF2 launch options to write console to log file
    (optional) Add "-conclearlog" to clear console.log file on game statup
        (If set, this program must be started after TF2, if not then doesn't matter)
        (If not set, the log file will keep growing and may need manual deleting after a while)



*/

mod logwatcher;
use logwatcher::*;

mod commander;

mod analyser;
use analyser::{Analyser};

mod regexes;
mod server;

mod timer;

fn main() {

    let print_console = false;

    let mut analyser = Analyser::new();

    // Setup watcher on log file
    let log_file = format!("{}/tf/console.log", analyser.serv.settings.directory);
    if let Ok(mut lw) = LogWatcher::register(log_file) {
        println!("Setup complete, happy gaming!");

        lw.watch(&mut move |line: String| {
            if print_console {
                println!("Console: {}", line);
            }
    
            analyser.update(&line);
        });
    } else {
        println!("No console.log file found. Please be sure to add -condebug to your launch options and then run the game before trying again.");
    }

}