use log::info;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rgtui::app::{App, AppResult};
use rgtui::event::{Event, EventHandler};
use rgtui::handler::{handle_key_events, handle_mouse_events};
use rgtui::tui::Tui;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;
use std::io;

fn main() -> AppResult<()> {
    // Initialize logging
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("rgtui.log").unwrap(),
    )
    .unwrap();

    info!("Starting rgtui...");

    // Create an application.
    let mut app = App::default();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    info!("Exiting...");
    tui.exit()?;
    Ok(())
}
