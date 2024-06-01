use ratatui::widgets::Paragraph;
use std::{error, process::Command};
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application state.
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Search text area.
    pub search_area: TextAreaToggle,

    /// Include files area.
    pub include_files_area: TextAreaToggle,

    // TODO: add files to exclude
    /// Keeping track of our text areas so we can toggle
    /// which one is active and which one isn't
    pub all_areas: Vec<TextAreaToggle>,

    /// Search result paragraph.
    pub search_res_par: Paragraph<'static>,
}

#[derive(Debug, Default, Clone)]
pub struct TextAreaToggle {
    pub area: TextArea<'static>,
    pub active: bool,
}

impl TextAreaToggle {
    fn new_active() -> Self {
        Self {
            area: TextArea::default(),
            active: true,
        }
    }
}

impl Default for App {
    /// Constructs a new instance of [`App`].
    fn default() -> Self {
        let mut app = App {
            running: true,
            search_area: TextAreaToggle::new_active(),
            include_files_area: TextAreaToggle::default(),
            search_res_par: Paragraph::default(),
            all_areas: vec![],
        };

        app.all_areas = vec![app.search_area.clone(), app.include_files_area.clone()];

        app
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
            .arg(&self.search_area.area.lines()[0])
            .output()
            .expect("error executing rg search");

        self.search_res_par = Paragraph::new(
            std::str::from_utf8(&res.stdout)
                .expect("could not convert rg search result to utf8 string")
                .to_string(),
        );
    }
}
