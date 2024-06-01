# rgtui

Simple TUI wrapper around [ripgrep](https://github.com/BurntSushi/ripgrep), inspired by VSCode's [project search](https://code.visualstudio.com/docs/editor/codebasics#_advanced-search-options)

It's written with:

- [ratatui](https://github.com/ratatui-org/ratatui)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [tui-textarea](https://github.com/rhysd/tui-textarea)
- ...

App layout (generated with [ratatui templates](https://github.com/ratatui-org/templates)):

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```

## TODO ripgrep options:

- `-F`: always interpret as literal string, not as regex
- `--heading`: This flag prints the file path above clusters of matches from each file instead of printing the file path as a prefix for each matched line.
- `--trim`: When set, all ASCII whitespace at the beginning of each line printed will be removed.
- `-i`: case insenstive search
  - Maybe `-S` instead?
- `-g`: glob searching, for including files (prepend `!` to ignore the pattern)
  - Globs are interpreted in exactly the same way as .gitignore patterns. That is, later globs will override earlier globs.
  - Maybe `--glob-case-insensitive` instead?
- `--line-number` if necessary
- `--json` if necessary
- `--sort` ?
- `--hidden` ?
- `--no-ignore` ?
