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

## TODO

### General features:

- [ ] move config to struct, allow configuration of settings in UI
- [ ] use JSON output and color-code based on that
- [ ] support multi-line search

### Possible `rg` options to support:

- [ ]`-g` or `--glob-case-insensitive`: glob searching, for including files (prepend `!` to ignore the pattern)
- [ ]`--json` if necessary
- [ ]`--sort`
- [ ]`--hidden`
- [ ]`--no-ignore`
