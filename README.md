# tf2_bot_kicker

## This project has been deprecated by [tf2-bot-kicker-gui](https://github.com/Bash-09/tf2-bot-kicker-gui)

An improved version featuring a GUI and many quality of life improvements for a much nicer experience.

## Original

A (somewhat) cross-platform bot identifier/kicker written in Rust.

A version with a Graphical User Interface can be found at the [tf2-bot-kicker-gui](https://github.com/Jenga500/tf2-bot-kicker-gui) repository, courtesy of [Jenga500](https://github.com/Jenga500)!


# Usage

Download the [command line program](https://github.com/Bash-09/tf2_bot_kicker/releases) or the [graphical user interface program](https://github.com/Jenga500/tf2-bot-kicker-gui/releases).

1. Add `bind F8 "exec command"` to your TF2 autoexec.cfg
2. Add `-condebug -conclearlog` to your Steam TF2 launch options. (Right click Team Fortress 2 in your Steam library, select Properties, and paste into the Launch Options section)
3. Make sure the TF2 directory is correct in `cfg/settings.cfg` or on the gui program
4. Launch TF2.
5. Run the program.

If the program fails to run, most likely it could not find your TF2 directory, ensure it is set correctly in cfg/settings.cfg. If this is troublesome, the program can also be run from directly inside the Team Fortress 2 folder without configuring the directory setting.

## Windows

Provided you have set the appropriate keybind in your TF2 `autoexec.cfg`, and have the directory correct in `settings.cfg`, the program should run without issue.

## Linux

Linux users may have to install some of the following packages. (As listed in the Ubuntu repository)

`libx11-dev`\
`libxtst-dev`\
`libudev-dev`\
`libinput-dev`

X11 environments should run fine once the appropriate packes are isntalled, you have set the appropriate keybind in your TF2 `autoexec.cfg`, and have the directory correct in `settings.cfg`.

### Wayland

Some Wayland environments will not allow `libinput` access to `/dev/uinput` without root.

To fix this you can either run the tf2_bot_kicker as root with sudo, or grant access to `/dev/uinput` with `chmod +0666 /dev/uinput` before running the program (This command will not persist after restart and will need to be run each time before using this program, for a more permanent solution you can follow the instructions under "Without X11" at https://crates.io/crates/tfc).


# Settings and Configuration

Inside the cfg folder is settings.cfg, you can change some basic settings here.\
`user` - Your SteamID3 (like from when you use the status command in-game) to indentify if bots are on the friendly or enemy team. (will stop attempting to kick enemy bots if set)\
`tf2_directory` - Location of the Team Fortress 2 folder. If the game is not installed in the default location you will have to change this. (Or you can run the program from in the TF2 folder without changing this setting.)\
`chat_reminders` - true/false if you want regular messages in chat to alert other players of current connected bots.\
`join_alerts` - true/false if you want chat messages that say when a bot is joining the server.\
`kick` - true/false if you want to automatically call votekicks on bots.\
`period` - Integer Time in seconds between actions (each alert/kick attempt)\
`key` - Which key the program will use to run commands. Most source engine key names will be recognized, if it doesn't recognize the key you entered it will default to F8 and print in the window when run.

Notes:
1. I encourage you to not leave chat_reminders on if the period is reasonably low (maybe 30 seconds?) as that may be annoying for the other players, find a balance or turn reminders off. I personally play with no chat_reminders or join_alerts at a period of 10 seconds.
2. It is recommended to use a key that isn't used much by other programs such as F8, as to avoid it potentially being annoying if it is pressed when you have the game tabbed-out or similar. 

# Building
This program should build without issue through Cargo on Windows, on Linux it should build provided the libraries listed above are installed.


# Additional Information

## Bot identification

Bots are identified by rules in the `cfg/bots.cfg` file.

This file has a few different sections:

`name:` - Under here are exact names to check for\
`regex:` - Checks for names that match the given regex, more reliable than the name section as if multiple of the same bot join the name will be prefixed with (1,2,3,etc) and won't match the name anymore, can also account for some cheeky names that have invisible characters hidden in them randomly etc.\
`uuid:` - Has SteamIDs of known bot accounts, this only supports SteamID3.\
`list:` - This has files which contain a number of SteamIDs to add to the list.\


## How it works
  
When you add `-condebug -conclearlog` to your TF2 launch options, it makes the game output the contents of the in-game console to a file called `console.log` in real-time. We can watch this file for certain details about the game such as players joining and connected, their SteamIDs, which team they are on (kind of) and how long they have been connected. The Source engine also has a nice feature of being able to run commands from cfg files from the console, so if we bind a key to run a particular cfg file, we can write our own commands to that file in the background and hit that bound key to run the commands. This program will automatically simulate the player pressing this button periodically so that it can run in-game commands like `status` and `tf_lobby_debug` to gather information, send chat messages, and call votekicks without need for any sort of hacks or illegal modifications to the game. Everything is just through config files and keyboard presses.

Using information from the `status` command, players have their names and SteamIDs checked against rules that are written in the `bots.cfg` file. If a player is identified as a bot, it is printed in the program window and does any actions you might have enabled in your `settings.cfg` like trying to kick them or alerting players of the bot's presence.
