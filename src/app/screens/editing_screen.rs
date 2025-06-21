use std::rc::Rc;

use ratatui::{
    crossterm::event::KeyCode,
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    style::{ Color, Style },
    text::Text,
    widgets::{ Block, Padding, Paragraph },
    Frame,
};

use crate::app::{ app::{ EditingSection, ScreenCode }, screens::{ screen_trait::Screen }, App };

pub struct EditingScreen {
    pub key_input: String,
    pub value_input: String,
    pub current_editing_section: EditingSection,
}

impl EditingScreen {
    pub fn new() -> EditingScreen {
        EditingScreen {
            key_input: String::new(),
            value_input: String::new(),
            current_editing_section: EditingSection::Key,
        }
    }

    /// Toggle whether the app is currently in editing mode. If the app is
    /// currently in editing mode, this will cycle the editing focus between
    /// the key and value sections. If the app is not in editing mode, this
    /// will move the app into editing mode with the focus on the key section.
    fn toggle_editing(&mut self) -> () {
        match &self.current_editing_section {
            EditingSection::Key => {
                self.current_editing_section = EditingSection::Value;
            }
            EditingSection::Value => {
                self.current_editing_section = EditingSection::Key;
            }
        }
    }

    /// Save the current key-value pair to the store and reset the input boxes.
    fn save(&mut self, app: &mut App) -> () {
        app.pairs.insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();

        self.current_editing_section = EditingSection::Key;
    }

    fn draw_header(&self, frame: &mut Frame, app: &mut App, area: Rect) -> () {
        let title_block: Block = Block::bordered()
            .style(Style::default())
            .padding(Padding::horizontal(2));

        let title: Paragraph = Paragraph::new(
            Text::styled(
                format!("{:?}", app.current_screen_code),
                Style::default().fg(Color::Green)
            )
        ).block(title_block);

        frame.render_widget(title, area);
    }

    fn draw_body(&self, frame: &mut Frame, area: Rect) -> () {
        let input_layout = Layout::default()
            .vertical_margin(1)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block: Block = Block::bordered()
            .title("Key")
            .title_alignment(Alignment::Center);
        let mut value_block: Block = Block::bordered()
            .title("Value")
            .title_alignment(Alignment::Center);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match self.current_editing_section {
            EditingSection::Key => {
                key_block = key_block.style(active_style);
            }
            EditingSection::Value => {
                value_block = value_block.style(active_style);
            }
        }

        let key_text: Paragraph = Paragraph::new(self.key_input.clone())
            .centered()
            .block(key_block);
        let value_text: Paragraph = Paragraph::new(self.value_input.clone())
            .centered()
            .block(value_block);

        frame.render_widget(key_text, input_layout[0]);
        frame.render_widget(value_text, input_layout[1]);
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) -> () {
        let outer_block: Block = Block::bordered().padding(Padding::horizontal(2));
        let editing_hint: Paragraph = Paragraph::new(
            "(ESC) to cancel/(Tab) to switch boxes/enter to complete"
        )
            .block(outer_block)
            .centered();

        frame.render_widget(editing_hint, area);
    }
}

impl Screen for EditingScreen {
    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> () {
        match key_code {
            // Save the current key-value pair.
            KeyCode::Enter => {
                match self.current_editing_section {
                    EditingSection::Key => {
                        self.current_editing_section = EditingSection::Value;
                    }
                    EditingSection::Value => {
                        self.save(app);
                        app.current_screen_code = ScreenCode::Main;
                    }
                }
            }
            // Delete the last character.
            KeyCode::Backspace => {
                match self.current_editing_section {
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
                self.current_editing_section = EditingSection::Key;
            }
            // Switch between key and value pairs.
            KeyCode::Tab => {
                self.toggle_editing();
            }
            // Append the newly added character.
            KeyCode::Char(input_char) => {
                match self.current_editing_section {
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
    fn ui(&mut self, app: &mut App, frame: &mut Frame) -> () {
        let layout: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());

        self.draw_header(frame, app, layout[0]);
        self.draw_body(frame, layout[1]);
        self.draw_footer(frame, layout[2]);
    }
}
