use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Execute rg search
        // TODO: search on each keystroke instead of on enter
        KeyCode::Enter => {
            if !app.text_area.lines()[0].is_empty() {
                app.search();
            }
        }
        // Update text box
        _ => {
            app.text_area.input(key_event);
        }
    }
    Ok(())
}
