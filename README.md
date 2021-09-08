# tf2_bot_kicker
A (mostly) cross-platform bot identifier/kicker written in Rust.

A version with a Graphical User Interface can be downloaded from the [tf2-bot-kicker-gui](https://github.com/Jenga500/tf2-bot-kicker-gui) repository, making everything much easier!

# What it does
When you run the program, it stores a collection names, steamids and rules from `bots.cfg` that identify bots. While you are playing a game of casual TF2 with this program running, it periodically checks the users connected to the server. If any of the players fit the bot-identifying information it has recorded it is able to send chat messages to warn other players of the bots joining or are currently on the server and automatically call vote-kicks against these bots. Several settings are provided to customise if you want it to automatically send chat messages, call vote kicks, how often this happens etc.

# Usage
No installation is required.
Download the program [here](https://github.com/Googe14/tf2_bot_kicker/releases)

tl;dr:
1. Add `bind F7 "exec command"` to your TF2 autoexec.cfg
2. Add `-condebug -conclearlog` to your Steam TF2 launch options.
3. Make sure the TF2 directory is correct in `cfg/settings.cfg`
4. Launch TF2.
5. Run the program.

Ensure when you run the program it is accompanied by it's cfg folder with at least a settings.cfg and bots.cfg file.

Before using the program, you'll need to bind your F7 key to run command.cfg, this is done easiest by putting `bind F7 "exec command"` in your autoexec.cfg file and either restarting the game or running `exec autoexec` in the console.

You will also need to add `-condebug` to your game launch options. In your Steam library right click Team Fortress 2 -> Properties and paste it into launch options. This will make the game print the output in the console to a console.log file so the program can keep track of what's happening in the game.
(Recommended) Optionally you can add `-conclearlog` to your launch options as well to clear this log file whenever the game is restarted. This will mean you will have to launch the bot-kicker *after* you start TF2, but otherwise the console.log file will just get longer and longer.

If the program fails to launch, it probably couldn't find your TF2 folder (This can be verified by running it from a cmd instead of directly). Check in `cfg/settigs.cfg` that the folder location is correct for your installation. The program can also be run directly in the Team Fortress 2 folder with disregard to whatever directory is set.

# Settings and config
Inside the cfg folder is settings.cfg, you can change some basic settings here.\
`user` - Add your SteamID3 (like from when you use the status command in-game) to stop trying to kick bots on the enemy team.\
`tf2_directory` - Is the location where TF2 is installed. If the game is not installed in the default location you will have to change this. (Or you can run the program in the TF2 folder without changing this setting.)\
`chat_reminders` - true/false if you want regular messages in chat to alert other players of current connected bots.\
`join_alerts` - true/false if you want chat messages that say when a bot is joining the server (This can occasionally miss bots if the period is set too high).\
`kick` - true/false if you want to automatically call votekicks on bots. (There is no way to determine when a vote is running or if you are on cooldown, so it may attempt to call votekicks even when you cannot)\
`period` - Integer Time in seconds between server refreshes (each alert/kick attempt)\
Note: I encourage you to not set the period too low with chat_reminders on as that may be annoying for the other players, find a balance or turn reminders off. I personally play with join_alerts on but no chat_reminders at a period of 10 seconds.

# Adding bots to the list
In the cfg folder there is a file called bots.cfg, this has all the information to identify the bots under a few sections.\
`name:` has exact names to check for.\
`regex:` has regexes to match player names against. (Be careful adding to these, it is easy to accidentally make a rule that will catch many regular players.)\
`uuid:` identifies specific steam accounts that are know to belong to bots.\
`list:` has files that contain uuids of bot accounts.\

If you encounter a bot that doesn't get picked up automatically, just look for the account's steamid or uuid as shown when you use the `status` command and add it under the uuid section in `bots.cfg`, this will also be easily found in the console.log file your tf folder.

When adding an external list, just put it in the cfg folder and add the filename to the list section of bots.cfg, the file doesn't need to be in any particular format or order, as long as it lists steamids as \[U:<zero-width space>x:xxxxx\] etc. The lists that are already there are available online, I found them in the [pazerOP](https://github.com/PazerOP/tf2_bot_detector) repository.

# Commands
When the program is running it'll periodically simulate the keyboard pressing F7, if this key is used for anything or might cause trouble by being pressed when the game is minimised etc, you can pause/resume the program by entering `echo pause` or `echo resume` in the TF2 console. Or you can just close and reopen the program, it just picks up again regardless of if you're in a match or anything so it doesn't really matter.

# Credits
This project makes use of modified code from the [Logwatcher](https://github.com/aravindavk/logwatcher) crate.
