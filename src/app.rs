use log::info;
use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, ScrollbarState},
};
use serde_jsonlines::JsonLinesReader;
use std::{error, io::Cursor, process::Command};
use tui_textarea::TextArea;

use crate::message::{Data, Message};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application state.
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Search result paragraph.
    pub search_res_par: ParagraphState,

    /// Keeping track of our text areas
    pub all_areas: Vec<TextArea<'static>>,

    /// Keep track what textarea index is active
    pub active: usize,

    pub scrollbar_vert_state: ScrollbarState,
}

pub struct ParagraphState {
    pub paragraph: Paragraph<'static>,
    pub current_scroll_index: u16,
    pub max_scroll_index: u16,
}

impl ParagraphState {
    pub fn new(
        paragraph: Paragraph<'static>,
        current_scroll_index: u16,
        max_scroll_index: u16,
    ) -> Self {
        ParagraphState {
            paragraph,
            current_scroll_index,
            max_scroll_index,
        }
    }
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
        area_files_exclude.set_cursor_line_style(Style::default());
        area_files_exclude.set_cursor_style(Style::default());
        area_files_exclude.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray))
                .title("Paths to exclude"),
        );

        let scrollbar_vert_state = ScrollbarState::new(0).position(0);

        Self {
            running: true,
            search_res_par: ParagraphState {
                paragraph: Paragraph::default(),
                current_scroll_index: 0,
                max_scroll_index: 0,
            },
            all_areas: vec![area_search, area_files_include, area_files_exclude],
            active: 0,
            scrollbar_vert_state,
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

        let mut cmd = Command::new("rg");
        cmd.arg("--fixed-strings") // Treat all patterns as literals instead of as regular expressions.
            .arg("--heading") // This flag prints the file path above clusters of matches from each file instead of printing the file path as a prefix for each matched line.
            .arg("--trim") // When set, all ASCII whitespace at the beginning of each line printed will be removed.
            .arg("--ignore-case") // When  this  flag  is  provided,  all patterns will be searched case insensitively.
            .arg("--line-number") // Show line numbers (1-based).
            .arg("-g") // Include or exclude files and directories for searching that match the given glob.
            .arg(include)
            .arg("-g") // Include or exclude files and directories for searching that match the given glob.
            .arg(exclude)
            .arg("--json")
            .arg(&self.all_areas[0].lines()[0]);

        info!("Executing ripgrep with args: {:?}", cmd.get_args());

        let output = cmd.output().expect("error executing rg search").stdout;
        let buff = Cursor::new(output);
        let reader = JsonLinesReader::new(buff);
        let messages = reader
            .read_all::<Message>()
            .collect::<std::io::Result<Vec<_>>>()
            .unwrap();

        let mut output: Vec<Line> = vec![];

        let mut old_fn = "";
        // TODO: set submatch to highlight red
        for mtch in messages.iter().filter_map(|msg| {
            if let Message::Match(m) = msg {
                Some(m)
            } else {
                None
            }
        }) {
            let file_name = match mtch.path.as_ref().unwrap() {
                Data::Text { text } => text,
                Data::Bytes { .. } => todo!("handle non-utf8 shizzle"),
            };

            if file_name != old_fn {
                output.push(Line::from(Span::styled(
                    format!("{}\n", file_name),
                    Style::new().magenta(),
                )));
                old_fn = file_name;
            }

            let matched_line = match &mtch.lines {
                Data::Text { text } => text,
                Data::Bytes { .. } => todo!("handle non-utf8 shizzle"),
            };

            let line_number =
                Span::styled(mtch.line_number.unwrap().to_string(), Style::new().green());

            output.push(Line::from(vec![
                line_number,
                Span::raw(":"),
                Span::raw(matched_line.clone()),
            ]));
        }

        let len = output.len();

        self.search_res_par = ParagraphState {
            paragraph: Paragraph::new(output),
            current_scroll_index: 0,
            max_scroll_index: len as u16,
        };

        self.scrollbar_vert_state = self.scrollbar_vert_state.content_length(len);
    }
}
