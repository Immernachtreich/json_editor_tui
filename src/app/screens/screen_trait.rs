use ratatui::{ crossterm::event::KeyCode, Frame };

use crate::app::App;

pub trait Screen {
    fn ui(&mut self, app: &mut App, frame: &mut Frame) -> ();
    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> ();
}
