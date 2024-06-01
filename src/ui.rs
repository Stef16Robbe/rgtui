use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType},
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

    app.search_area
        .area
        .set_placeholder_text("Start typing to search...");
    app.search_area.area.set_block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title("rgtui")
            .title_alignment(Alignment::Center),
    );

    app.include_files_area
        .area
        .set_placeholder_text("Files to include");
    app.include_files_area
        .area
        .set_block(Block::bordered().border_type(BorderType::Rounded));

    // TODO: fix this
    let search_res_par = app.search_res_par.clone();
    let search_res_par = search_res_par
        .alignment(Alignment::Left)
        .block(Block::bordered().border_type(BorderType::Rounded));

    if let Some(at) = app.all_areas.iter_mut().find(|at| at.active) {
        at.area.set_block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().blue()),
        )
    };

    frame.render_widget(app.search_area.area.widget(), bar_left[0]);
    frame.render_widget(app.include_files_area.area.widget(), bar_left[1]);
    frame.render_widget(search_res_par, search_res_right);
}
