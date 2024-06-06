use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use tui_textarea::TextArea;

fn deactivate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .title(" Inactive (TAB to switch) "),
    );
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Active "),
    );
}

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
            deactivate(&mut app.all_areas[app.active]);
            app.active = (app.active + 1) % 2;
            activate(&mut app.all_areas[app.active]);
        }
        // Search on keystrokes
        _ => {
            app.all_areas[app.active].input(key_event);
            if !app.all_areas[0].lines()[0].is_empty() {
                app.search();
            } else {
                app.search_res_par = Paragraph::default();
            }
        }
    }
    Ok(())
}
