# Nimbus Text Editor

Nimbus is a terminal-based text editor. It serves as my personal study project.

**Features**
- Basic text editing
- Incremental Search
- Basic syntax highlighting
- Basic Vim-like motions

Supported file types for highlighting:
- Rust
- toml


## Documentation

**Keybinds**

| Keybind       | Description |
|-------------- | -------------- |
| :         | Start a vim-like command input.     |
| Ctrl-w    | Write changes to file.     |
| Ctrl-q    | Quit.     |
| Arrows    | Vertical/Horizontal Navigation |
| h,j,k,l   | Vertical/Horizontal Navigation |
| Home      | Go to start of line.     |
| End       | Go to end of line.     |

**Commands**

| Command | Description |
|-------------- | -------------- |
| w | Same as Ctrl-w |
| q | Same as Ctrl-q |


## Implementation

**Piece Table**

The text buffer is implemented with a piece table data structure.
It allows fast insertion and deletion times.
It also does not require much meta-data per line to be stored.

## References:
[Termion - Rust terminal library](https://docs.rs/termion/latest/termion/)
[Piece Table wikipedia](https://en.wikipedia.org/wiki/Piece_table)  
[Simple Explanation of Piece Table](https://darrenburns.net/posts/piece-table/)  
[Piece table implementation in JavaScript](https://github.com/sparkeditor/piece-table)  
