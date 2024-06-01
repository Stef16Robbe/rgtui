use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());

    app.text_area
        .set_placeholder_text("Start typing to search...");
    app.text_area.set_block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("rgtui")
            .title_alignment(Alignment::Center),
    );

    // TODO: fix this
    let res_box = app.search_res.clone();
    let res_box = res_box
        .alignment(Alignment::Left)
        .block(Block::bordered().border_type(BorderType::Rounded));

    frame.render_widget(app.text_area.widget(), main_layout[0]);
    frame.render_widget(res_box, main_layout[1]);
}
