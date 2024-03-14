# Nimbus Text Editor

Nimbus is a terminal-based text editor. It is written in rust and supports basic functionality for writing text or code.
It is also a personal study project of mine, meant to sate my curiosity of few topics (terminal API, text data structures, highlighting and vim motions).

**Features**
- Text editing
- Incremental Search
- Syntax highlighting
- Some vim motions

Supported file types for highlighting:
- Rust
- toml


## Documentation

**Keybinds**

| Keybind       | Description |
|-------------- | -------------- |
| Ctrl-w        | Write changes to file.    |
| Ctrl-q        | Quit.                     |
| I             | Enter input mode.         |
| Esc           | Exit input mode.          |
| :             | Enter command input.      |
| Arrows        | Vertical/Horizontal Navigation    |
| h,j,k,l       | Vertical/Horizontal Navigation    |
| Backspace     | Delete a character before cursor. |
| Delete        | Delete a character after cursor.  |
| Home          | Go to start of line.      |
| End           | Go to end of line.        |
| 0             | Go to start of line.      |
| $             | Go to end of line.        |
| A             | Go to end of line and change to INSERT mode.|

**Commands**

| Command | Description |
|-------------- | -------------- |
| w     | Same as Ctrl-w |
| q     | Same as Ctrl-q |
| /     | Search a string |
| debug | Toggle debug bar |
| help  | Show help text. |


## Implementation

**Piece Table**

The text buffer is implemented with a piece table data structure.
It allows fast insertion and deletion times.
It also does not require much meta-data per line to be stored.

## References:
[Termion - Rust terminal library](https://docs.rs/termion/latest/termion/)
[Vim motions](https://vimdoc.sourceforge.net/htmldoc/motion.html)
[Piece Table wikipedia](https://en.wikipedia.org/wiki/Piece_table)  
[Simple Explanation of Piece Table](https://darrenburns.net/posts/piece-table/)  
[Piece table implementation in JavaScript](https://github.com/sparkeditor/piece-table)  
