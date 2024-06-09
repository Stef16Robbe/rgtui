use log::info;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    widgets::{Block, BorderType, Scrollbar, ScrollbarOrientation},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());

    let search_res_right = main_layout[1];

    let bar_left = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
        ])
        .split(main_layout[0]);

    let search_res_par = app
        .search_res_par
        .paragraph
        .clone() // FIXME
        .alignment(Alignment::Left)
        .block(Block::bordered().border_type(BorderType::Rounded));

    for (i, area) in app.all_areas.iter().enumerate() {
        let widget = area.widget();
        frame.render_widget(widget, bar_left[i]);
    }

    frame.render_widget(search_res_par, search_res_right);

    let scrollbar_vert = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    info!("Rendering scroll bar: {:?}", &app.scrollbar_vert_state);

    frame.render_stateful_widget(
        scrollbar_vert,
        search_res_right.inner(&Margin {
            // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.scrollbar_vert_state,
    );
}
