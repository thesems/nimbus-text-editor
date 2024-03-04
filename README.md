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

## Implementation
**Piece Table**
The text buffer is implemented with a piece table data structure.
It allows fast insertion and deletion times.
It also does not require much meta-data per line to be stored.

### References:
[Simple Explanation of Piece Table](https://darrenburns.net/posts/piece-table/)
[Piece Table wikipedia](https://en.wikipedia.org/wiki/Piece_table)
[Piece table implementation in JavaScript](https://github.com/sparkeditor/piece-table)
