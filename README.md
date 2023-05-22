# rs2048 - 2048 clone in rust
    This is a dumb coding challenge for me to exercise doing rust things in.
Features:
 * Interactive / TUI interface for 2048
 * Game logic largely works.
 * Fun Merge/Validation Checking code
TODO:
 * Make game menu / restart UI
 * make TUI spacing prettier / consistent. (learn string formatting loesr)
 * Make controls screen / overrideable mappings via text file / TUI
 * uh 2048 detection / loss detection (this is easy)
 * Make graphical interface
 * Save game state.
 * leaderboard --> global leaderboard on my servers(?)
 * timer? 
 * variable size play board (basically replace 4  everywhere with a variable lol)
## Building
Just run
`
cargo build
`

and it should work.
Currently if you run, you just get tossed into a game.