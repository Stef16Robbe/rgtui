use crate::app::{App, AppResult, ParagraphState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use log::info;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};
use tui_textarea::TextArea;

fn deactivate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        textarea
            .block()
            .unwrap()
            .clone()
            .style(Style::default().fg(Color::DarkGray)),
    );
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(textarea.block().unwrap().clone().style(Style::default()));
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    info!("Handling key event: {:?}", key_event);
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
            app.active = (app.active + 1) % 3;
            activate(&mut app.all_areas[app.active]);
        }
        // Search on keystrokes
        _ => {
            app.all_areas[app.active].input(key_event);
            if !app.all_areas[0].lines()[0].is_empty() {
                app.search();
            } else {
                app.search_res_par.paragraph = Paragraph::default();
            }
        }
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> AppResult<()> {
    info!("Handling mouse event: {:?}", mouse_event);
    match mouse_event.kind {
        MouseEventKind::ScrollDown => {
            let scroll_idx = (app.search_res_par.current_scroll_index + 1)
                .min(app.search_res_par.max_scroll_index - 1);

            app.search_res_par = ParagraphState::new(
                app.search_res_par.paragraph.clone().scroll((scroll_idx, 0)),
                scroll_idx,
                app.search_res_par.max_scroll_index,
            )
        }
        MouseEventKind::ScrollUp => {
            let scroll_idx = app
                .search_res_par
                .current_scroll_index
                .checked_sub(1)
                .unwrap_or(0);

            app.search_res_par = ParagraphState::new(
                app.search_res_par.paragraph.clone().scroll((scroll_idx, 0)),
                scroll_idx,
                app.search_res_par.max_scroll_index,
            )
        }
        _ => (),
    }
    Ok(())
}
