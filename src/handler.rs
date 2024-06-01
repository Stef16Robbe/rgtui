use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::Paragraph;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Search on keystrokes
        _ => {
            app.text_area.input(key_event);
            if !app.text_area.lines()[0].is_empty() {
                app.search();
            } else {
                app.search_res = Paragraph::default();
            }
        }
    }
    Ok(())
}
