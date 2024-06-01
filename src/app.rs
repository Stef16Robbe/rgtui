use ratatui::widgets::Paragraph;
use std::{error, process::Command};
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Search text area.
    pub text_area: TextArea<'static>,
    /// Search result paragraph.
    pub search_res: Paragraph<'static>,
}

impl Default for App {
    /// Constructs a new instance of [`App`].
    fn default() -> Self {
        Self {
            running: true,
            text_area: TextArea::default(),
            search_res: Paragraph::default(),
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
        let res = Command::new("rg")
            .arg("--fixed-strings") // Treat all patterns as literals instead of as regular expressions.
            .arg("--heading") // This flag prints the file path above clusters of matches from each file instead of printing the file path as a prefix for each matched line.
            .arg("--trim") // When set, all ASCII whitespace at the beginning of each line printed will be removed.
            .arg("--ignore-case") // When  this  flag  is  provided,  all patterns will be searched case insensitively.
            .arg("--line-number") // Show line numbers (1-based).
            .arg(&self.text_area.lines()[0])
            .output()
            .expect("error executing rg search");

        self.search_res = Paragraph::new(
            std::str::from_utf8(&res.stdout)
                .expect("could not convert rg search result to utf8 string")
                .to_string(),
        );
    }
}
