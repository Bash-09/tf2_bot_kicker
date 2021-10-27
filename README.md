# tf2_bot_kicker
A (mostly) cross-platform bot identifier/kicker written in Rust.
(May not support Wayland)

A version with a Graphical User Interface can be found at the [tf2-bot-kicker-gui](https://github.com/Jenga500/tf2-bot-kicker-gui) repository, courtesy of [Jenga500](https://github.com/Jenga500)!


# Usage

Download the [command line program](https://github.com/Googe14/tf2_bot_kicker/releases) or the [graphical user interface program](https://github.com/Jenga500/tf2-bot-kicker-gui/releases).

1. Add `bind F7 "exec command"` to your TF2 autoexec.cfg
2. Add `-condebug -conclearlog` to your Steam TF2 launch options. (Right click Team Fortress 2 in your Steam library, select Properties, and paste into the Launch Options section)
3. Make sure the TF2 directory is correct in `cfg/settings.cfg` or on the gui program
4. Launch TF2.
5. Run the program.

If the program fails to run, most likely it could not find your TF2 directory, ensure it is set correctly in cfg/settings.cfg. If this is troublesome, the program can also be run from directly inside the Team Fortress 2 folder without configuring the directory setting.

## Linux devices may have ensure they have installed additional libraries installed

### Ubuntu packages:

`libx11-dev`\
`libxtst-dev`\
`libudev-dev`\
`libinput-dev`

# Settings and Configuration

Inside the cfg folder is settings.cfg, you can change some basic settings here.\
`user` - Your SteamID3 (like from when you use the status command in-game) to indentify if bots are on the friendly or enemy team. (will stop attempting to kick enemy bots if set)\
`tf2_directory` - Location of the Team Fortress 2 folder. If the game is not installed in the default location you will have to change this. (Or you can run the program from in the TF2 folder without changing this setting.)\
`chat_reminders` - true/false if you want regular messages in chat to alert other players of current connected bots.\
`join_alerts` - true/false if you want chat messages that say when a bot is joining the server.\
`kick` - true/false if you want to automatically call votekicks on bots.\
`period` - Integer Time in seconds between actions (each alert/kick attempt)\
Note: I encourage you to not leave chat_reminders on if the period is reasonably low (maybe 30 seconds?) as that may be annoying for the other players, find a balance or turn reminders off. I personally play with join_alerts on but no chat_reminders at a period of 10 seconds.


# Building
This program should build without issue through Cargo on Windows, on Linux it should build provided the libraries listed above are installed.


# Additional Information

## Bot identification

Bots are identified by rules in the `cfg/bots.cfg` file.

This file has a few different sections:

`name:` - Under here are exact names to check for\
`regex:` - Checks for names that match the given regex, more reliable than the name section as if multiple of the same bot join the name will be prefixed with (1,2,3,etc) and won't match the name anymore, can also account for some cheeky names that have invisible characters hidden in them randomly etc.\
`uuid:` - Has SteamIDs of known bot accounts, this currently only supports SteamID3.\
`list:` - This has files which contain a number of SteamIDs to add to the list.\


## How it works
  
By adding `-condebug -conclearlog` to your TF2 launch options, the game outputs all contents of the in-game console to a file `console.log` in real-time (and also clears it when you start the game so it doesn't get too big). This program watches that file to get some live information about the game, including the steamid, name and (kind of) team of each player if the `status` and `tf_lobby_debug` commands are run. Since your F7 key is bound to `"exec command"`, if there are commands written to the file commands.cfg then they will be run when the F7 key is pressed, that means this program can then run commands in the game by writing commands to that file and simulating the player pressing the key. Periodically, the program runs the `status` and `tf_lobby_debug` commands to get information on the current players, checks if any of them are bots according to the rules specified in `bots.cfg`, and if they are then it runs commands to send messages in chat and/or call a votekick on accounts identified as bots. That is all, no hacked client required!

# Credits
This project makes use of modified code from the [Logwatcher](https://github.com/aravindavk/logwatcher) crate.
