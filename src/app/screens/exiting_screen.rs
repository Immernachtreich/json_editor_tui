use ratatui::crossterm::event::KeyCode;

use crate::app::{ screens::screen_trait::Screen, App };

#[derive(Default)]
pub struct ExitingScreen;

impl Screen for ExitingScreen {
    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> () {
        match key_code {
            KeyCode::Char('y') => {
                app.should_print = true;
                app.should_exit = true;
            }
            KeyCode::Char('n') | KeyCode::Char('q') => {
                app.should_print = true;
                app.should_exit = true;
            }
            _ => (),
        }
    }
    fn ui(&mut self, app: &mut crate::app::App, frame: &mut ratatui::Frame) -> () {
        todo!()
    }
}
