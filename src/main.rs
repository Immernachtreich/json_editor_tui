use std::{ error::Error, io::{ self, Stderr } };

use ratatui::{
    crossterm::{
        event::{ DisableMouseCapture, EnableMouseCapture },
        execute,
        terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    },
    prelude::CrosstermBackend,
    Terminal,
};

mod app;
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut std_err: Stderr = io::stderr();
    execute!(std_err, EnterAlternateScreen, EnableMouseCapture)?;

    let backend: CrosstermBackend<Stderr> = CrosstermBackend::new(std_err);
    let mut terminal: Terminal<CrosstermBackend<Stderr>> = Terminal::new(backend)?;

    let mut app: App = App::new();
    let result = app.run_app(&mut terminal);

    // restore terminal to previous state
    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    terminal.show_cursor()?;

    match result {
        Ok(should_print) if should_print => {
            app.print_json()?;
        }
        Err(err) => println!("{err:?}"),
        _ => {}
    }

    Ok(())
}
