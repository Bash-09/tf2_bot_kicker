# tf2_bot_kicker
My Implementation of a bot identifier/kicker in Rust. This was designed to be cross-platform and I don't believe it uses any platform specific libraries, but this has only been tested on Windows so if it doesn't work on Linux then I'll figure that out another time.

# Usage
tl;dr:
1. Add `bind F7 "exec command"` to autoexec.cfg
2. Add `-condebug -conclearlog` to your Steam TF2 launch options.
3. Launch TF2.
4. Run this program.

Either download the compiled binary in the zip file or download the source and compile it yourself. Make sure the bots.txt file is in the same directory and just run it. It will attempt to search for your TF2 directory in the default locations on Windows or Linux. If you have installed TF2 in a different location, run the program from inside the Team Fortress 2 folder.

Before using the program, you'll need to bind your F7 key to run command.cfg, this is done easiest by putting `bind F7 "exec command"` in your autoexec.cfg file and either restarting the game or running `exec autoexec` in the console.

You will also need to add `-condebug` to your game launch options. In your Steam library right click Team Fortress 2 -> Properties and paste it into launch options. This will make the game print the output in the console to a console.log file so the program can keep track of what's happening in the game.
(Recommended) Optionally you can add `-conclearlog` to your launch options as well to clear this log file whenever the game is restarted. This will mean you will have to launch the bot-kicker *after* you start TF2, but otherwise the console.log file will just get longer and longer.

The `bots.txt` contains the names to match through either direct string match or regex. The program will automatically identify people in the game who match the names or regexes in bots.txt (so be careful about adding names with characters like . * + etc).

# Commands
When the program is running it'll periodically simulate the keyboard pressing F7, if this key is used for anything or might cause trouble by being pressed when the game is minimised etc, you can pause/resume the program by entering `echo pause` or `echo resume` in the TF2 console. Or you can just close and reopen the program, it just picks up again regardless of if you're in a match or anything so it doesn't really matter.

# Known issues / TODO
1. Program will try to kick bots on other team + tries to call votes when on cooldown. Currently the program does not check if the bot is on the same team as it does not identify which user is you (yet). This could be fixed by having the user supply their uuid, but I am hoping to leave as little work to the user as possible so maybe if I think of another way, or maybe make supplying your own uuid an optional thing.

2. Storage of bot names is mixed regex and string, maybe I'll add a way to specify which is which just to be safe, but currently I don't believe it'll cause too many problems unless a bot has a weird name.

# Credits
This project makes use of modified code from the [Logwatcher](https://github.com/aravindavk/logwatcher) crate.
