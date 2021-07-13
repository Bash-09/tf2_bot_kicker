# tf2_bot_kicker
My Implementation of a bot identifier/kicker in Rust. This was designed to be cross-platform and I don't believe it uses any platform specific libraries, but this has only been tested on Windows so if it doesn't work on Linux then I'll figure that out another time.

# Usage
No installation is required.
Download the program [here](https://github.com/Googe14/tf2_bot_kicker/releases/tag/v1.0.1)

tl;dr:
1. Add `bind F7 "exec command"` to autoexec.cfg
2. Add `-condebug -conclearlog` to your Steam TF2 launch options.
3. Launch TF2.
4. Run the program.

Ensure when you run the program it is accompanied by it's cfg folder with at least a settings.cfg and bots.cfg file.

Before using the program, you'll need to bind your F7 key to run command.cfg, this is done easiest by putting `bind F7 "exec command"` in your autoexec.cfg file and either restarting the game or running `exec autoexec` in the console.

You will also need to add `-condebug` to your game launch options. In your Steam library right click Team Fortress 2 -> Properties and paste it into launch options. This will make the game print the output in the console to a console.log file so the program can keep track of what's happening in the game.
(Recommended) Optionally you can add `-conclearlog` to your launch options as well to clear this log file whenever the game is restarted. This will mean you will have to launch the bot-kicker *after* you start TF2, but otherwise the console.log file will just get longer and longer.

# Settings and config
Inside the cfg directory is settings.cfg. This file has a few basic settings like if to alert other players of bots via chat messages, or if to automatically try and kick bots, even how frequently it will do so (in seconds). Here you can also add your own SteamID3 so the program know who you are (This will stop the program from trying to kick bots on the enemy team.)

# Adding bots to the list
In the cfg folder there is a file called bots.cfg, this has all the information to identify the bots under a few sections.\
`name:` has exact names to check for.\
`regex:` has regexes to match player names against.\
`uuid:` identifies specific steam accounts that are know to belong to bots.\
`list:` has files that contain uuids of bot accounts.\

When adding an external list, just put it in the cfg folder and add the filename to the list section of bots.cfg, the file doesn't need to be in any particular format or order, as long as it lists steamids as \[U:<zero-width space>x:xxxxx\] etc. The lists that are already there are available online, I found them in the [pazerOP](https://github.com/PazerOP/tf2_bot_detector) repository.

# Commands
When the program is running it'll periodically simulate the keyboard pressing F7, if this key is used for anything or might cause trouble by being pressed when the game is minimised etc, you can pause/resume the program by entering `echo pause` or `echo resume` in the TF2 console. Or you can just close and reopen the program, it just picks up again regardless of if you're in a match or anything so it doesn't really matter.

# Credits
This project makes use of modified code from the [Logwatcher](https://github.com/aravindavk/logwatcher) crate.
