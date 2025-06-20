use ratatui::crossterm::event::KeyCode;

use crate::app::{ app::{ EditingSection, ScreenCode }, screens::screen_trait::Screen, App };

#[derive(Default)]
pub struct MainScreen;

impl Screen for MainScreen {
    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> () {
        match key_code {
            KeyCode::Char('e') | KeyCode::Char('E') => {
                // Set the current screen to editing
                app.current_screen_code = ScreenCode::Editing;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                // Exit the application
                app.current_screen_code = ScreenCode::Exiting;
            }
            _ => {}
        }
    }
    fn ui(&mut self, app: &mut App, frame: &mut ratatui::Frame) -> () {
        todo!()
    }
}
