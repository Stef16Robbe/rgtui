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

    pub fn search(&mut self) {
        // TODO: add support for complex search
        let res = Command::new("rg")
            .arg(&self.text_area.lines()[0]) // TODO: support multi-line search?
            .output()
            .expect("error searching with rg");

        self.search_res = Paragraph::new(
            std::str::from_utf8(&res.stdout)
                .expect("could not convert res to utf8 string")
                .to_string(),
        );
    }
}
