use crate::app::{App, AppResult, TextAreaToggle};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::Paragraph;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Exit application on `Ctrl-C`
    if key_event.modifiers == KeyModifiers::CONTROL && key_event.code == KeyCode::Char('c') {
        app.quit();
    }

    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        // For now, disable creation of multiple lines as multi-line search is not supported
        KeyCode::Enter => (),
        // Toggle active fields
        KeyCode::Tab => {
            // TODO: fix this
            let mut all_areas = app.all_areas.clone();
            // find current active field
            let active = all_areas
                .iter_mut()
                .enumerate()
                .find(|(_, at)| at.active)
                .expect("at least one text area needs to be active");
            // set it to inactive
            active.1.active = false;
            // set next field in the list on active
            // next as in literally n + 1 because currently the fields are ordered from top to bottom
            app.all_areas[active.0 + 1].active = true;
        }
        // Search on keystrokes
        _ => {
            // find the active textarea so we can change it's input
            // this is O(n), but we have a low amount of fields so for now this is OK
            let active: &mut TextAreaToggle = app
                .all_areas
                .iter_mut()
                .find(|at| at.active)
                .expect("at least one text area needs to be active");

            active.area.input(key_event);
            if !app.search_area.area.lines()[0].is_empty() {
                app.search();
            } else {
                app.search_res_par = Paragraph::default();
            }
        }
    }
    Ok(())
}
