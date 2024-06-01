use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let style: Style = Style::default().fg(Color::White).bg(Color::Black);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.size());

    app.text_area.set_style(style);
    // search_box.set_cursor_line_style(Style::default());
    app.text_area.set_placeholder_text("Search...");

    // TODO: fix this
    let res_box = app.search_res.clone();
    let res_box = res_box.alignment(Alignment::Left).style(style);

    frame.render_widget(app.text_area.widget(), main_layout[0]);
    frame.render_widget(res_box, main_layout[1]);
}
