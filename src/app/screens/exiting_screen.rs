use ratatui::{
    crossterm::event::KeyCode,
    style::{ Color, Style },
    text::Text,
    widgets::{ Block, Clear, Paragraph, Wrap },
};

use crate::app::{ screens::{ centered_rect, screen_trait::Screen }, App };

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
    fn ui(&mut self, _app: &mut App, frame: &mut ratatui::Frame) -> () {
        // Clear the terminal
        frame.render_widget(Clear, frame.area());

        let layout_block = Block::bordered()
            .title("Y/N")
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red)
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .centered()
            .block(layout_block)
            .wrap(Wrap { trim: false });

        let centered_area = centered_rect(60, 25, frame.area());

        frame.render_widget(exit_paragraph, centered_area);
    }
}
