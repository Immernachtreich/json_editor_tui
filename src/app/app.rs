use std::{ collections::HashMap, io };

use ratatui::{ crossterm::event::{ self, Event, KeyEventKind }, prelude::Backend, Frame, Terminal };

use crate::app::screens::{ EditingScreen, ExitingScreen, MainScreen, Screen };

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ScreenCode {
    Main,
    Editing,
    Exiting,
}

pub enum EditingSection {
    Key,
    Value,
}

pub struct App {
    pub pairs: HashMap<String, String>,
    pub current_screen_code: ScreenCode,
    pub screens: HashMap<ScreenCode, Box<dyn Screen>>,
    pub should_exit: bool,
    pub should_print: bool,
}

impl App {
    pub fn new() -> App {
        let mut screens: HashMap<ScreenCode, Box<dyn Screen>> = HashMap::with_capacity(3);

        screens.insert(ScreenCode::Main, Box::new(MainScreen::default()));
        screens.insert(ScreenCode::Editing, Box::new(EditingScreen::new()));
        screens.insert(ScreenCode::Exiting, Box::new(ExitingScreen::default()));

        App {
            pairs: HashMap::new(),
            current_screen_code: ScreenCode::Main,
            screens,
            should_exit: false,
            should_print: false,
        }
    }

    fn switch_screen(&mut self, screen_code: ScreenCode) -> () {
        self.current_screen_code = screen_code;
    }

    /// Print the contents of the key-value store as a JSON object.
    ///
    /// Will error if the contents of the store cannot be converted to a JSON
    /// string.
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        loop {
            if let Event::Key(key) = event::read()? {
                // Skip events that are not press events
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Move the screen out, leaving `None`
                if let Some(mut current_screen) = self.screens.remove(&self.current_screen_code) {
                    // Now we can hand &mut self to the screen safely
                    current_screen.handle_key_event(self, key.code);

                    // Put the screen back
                    self.screens.insert(self.current_screen_code.clone(), current_screen);
                }
            }

            if self.should_exit {
                break Ok(self.should_print);
            }

            terminal.draw(|frame: &mut Frame| self.draw(frame))?;
        }
    }

    pub fn draw(&self, frame: &mut Frame) {}
}
