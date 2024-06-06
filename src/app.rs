use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{error, process::Command};
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application state.
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Search result paragraph.
    pub search_res_par: Paragraph<'static>,

    /// Keeping track of our text areas
    pub all_areas: Vec<TextArea<'static>>,

    /// Keep track what index is active
    pub active: usize,
}

impl Default for App {
    /// Constructs a new instance of [`App`].
    fn default() -> Self {
        // Default to "active"
        let mut area_search = TextArea::default();
        area_search.set_placeholder_text("Start typing to search...");
        area_search.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
        area_search.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
        area_search.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title("Search term"),
        );

        // Default to "inactive"
        let mut area_files_include = TextArea::default();
        // area_files_include.set_placeholder_text("Files to include");
        area_files_include.set_cursor_line_style(Style::default());
        area_files_include.set_cursor_style(Style::default());
        area_files_include.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray))
                .title("Paths to include"),
        );

        // Default to "inactive"
        let mut area_files_exclude = TextArea::default();
        // area_files_exclude.set_placeholder_text("Files to exclude");
        area_files_exclude.set_cursor_line_style(Style::default());
        area_files_exclude.set_cursor_style(Style::default());
        area_files_exclude.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray))
                .title("Paths to exclude"),
        );

        Self {
            running: true,
            search_res_par: Paragraph::default(),
            all_areas: vec![area_search, area_files_include, area_files_exclude],
            active: 0,
        }
    }
}

impl App {
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Search the given term with rg.
    pub fn search(&mut self) {
        let include = &mut self.all_areas[1].lines()[0].to_owned();
        if !include.is_empty() && include.ends_with('/') {
            include.push_str("**");
        }

        let exclude = &mut self.all_areas[2].lines()[0].to_owned();
        if !exclude.is_empty() {
            exclude.insert(0, '!');

            if exclude.ends_with('/') {
                exclude.push_str("**");
            }
        }

        let res = Command::new("rg")
            .arg("--fixed-strings") // Treat all patterns as literals instead of as regular expressions.
            .arg("--heading") // This flag prints the file path above clusters of matches from each file instead of printing the file path as a prefix for each matched line.
            .arg("--trim") // When set, all ASCII whitespace at the beginning of each line printed will be removed.
            .arg("--ignore-case") // When  this  flag  is  provided,  all patterns will be searched case insensitively.
            .arg("--line-number") // Show line numbers (1-based).
            .arg("-g") // Include or exclude files and directories for searching that match the given glob.
            .arg(include)
            .arg("-g") // Include or exclude files and directories for searching that match the given glob.
            .arg(exclude)
            .arg(&self.all_areas[0].lines()[0])
            .output()
            .expect("error executing rg search");

        self.search_res_par = Paragraph::new(
            std::str::from_utf8(&res.stdout)
                .expect("could not convert rg search result to utf8 string")
                .to_string(),
        );
    }
}
