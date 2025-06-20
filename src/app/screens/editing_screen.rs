use ratatui::{ crossterm::event::KeyCode, Frame };

use crate::app::{ app::{ EditingSection, ScreenCode }, screens::screen_trait::Screen, App };

pub struct EditingScreen {
    pub key_input: String,
    pub value_input: String,
    pub current_editing_section: Option<EditingSection>,
}

impl EditingScreen {
    pub fn new() -> EditingScreen {
        EditingScreen {
            key_input: String::new(),
            value_input: String::new(),
            current_editing_section: None,
        }
    }

    /// Toggle whether the app is currently in editing mode. If the app is
    /// currently in editing mode, this will cycle the editing focus between
    /// the key and value sections. If the app is not in editing mode, this
    /// will move the app into editing mode with the focus on the key section.
    fn toggle_editing(&mut self) -> () {
        if let Some(edit_section) = &self.current_editing_section {
            match edit_section {
                EditingSection::Key => {
                    self.current_editing_section = Some(EditingSection::Value);
                }
                EditingSection::Value => {
                    self.current_editing_section = Some(EditingSection::Key);
                }
            }
        } else {
            // toggle to editing mode with the cursor set on the key section.
            self.current_editing_section = Some(EditingSection::Key);
        }
    }

    /// Save the current key-value pair to the store and reset the input boxes.
    pub fn save(&mut self, app: &mut App) -> () {
        app.pairs.insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();

        self.current_editing_section = None;
    }
}

impl Screen for EditingScreen {
    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> () {
        let current_section: &EditingSection = match &self.current_editing_section {
            Some(value) => value,
            None => {
                return;
            }
        };

        match key_code {
            // Save the current key-value pair.
            KeyCode::Enter => {
                match current_section {
                    EditingSection::Key => {
                        self.current_editing_section = Some(EditingSection::Value);
                    }
                    EditingSection::Value => {
                        self.save(app);
                        app.current_screen_code = ScreenCode::Main;
                    }
                }
            }
            // Delete the last character.
            KeyCode::Backspace => {
                match current_section {
                    EditingSection::Key => {
                        self.key_input.pop();
                    }
                    EditingSection::Value => {
                        self.value_input.pop();
                    }
                }
            }
            // Exit the editing screen.
            KeyCode::Esc => {
                app.current_screen_code = ScreenCode::Main;
                self.current_editing_section = None;
            }
            // Switch between key and value pairs.
            KeyCode::Tab => {
                self.toggle_editing();
            }
            // Append the newly added character.
            KeyCode::Char(input_char) => {
                match current_section {
                    EditingSection::Key => {
                        self.key_input.push(input_char);
                    }
                    EditingSection::Value => {
                        self.value_input.push(input_char);
                    }
                }
            }
            _ => {}
        }
    }
    fn ui(&mut self, app: &mut App, frame: &mut Frame) -> () {}
}
