use ratatui::crossterm::event::KeyCode;

use crate::app::{ screens::screen_trait::Screen, App };

#[derive(Default)]
pub struct ExitingScreen;

impl Screen for ExitingScreen {
    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> () {
        // match key_code {
        //     KeyCode::Char('y') => Some(true),
        //     KeyCode::Char('n') | KeyCode::Char('q') => Some(false),
        //     _ => None,
        // }
    }
    fn ui(&mut self, app: &mut crate::app::App, frame: &mut ratatui::Frame) -> () {
        todo!()
    }
}
