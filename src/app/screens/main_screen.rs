use std::{ collections::HashMap, rc::Rc };

use ratatui::{
    crossterm::event::KeyCode,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Color, Style, Stylize },
    text::{ Line, Span, Text },
    widgets::{ Block, Borders, List, ListItem, Padding, Paragraph },
    Frame,
};

use crate::app::{ app::{ ScreenCode }, screens::screen_trait::Screen, App };

#[derive(Default)]
pub struct MainScreen;

impl MainScreen {
    fn draw_header(&self, frame: &mut Frame, area: Rect) -> () {
        let title_block: Block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .padding(Padding::horizontal(2));

        let title: Paragraph = Paragraph::new(
            Text::styled("Create New JSON", Style::default().fg(Color::Green))
        ).block(title_block);

        frame.render_widget(title, area);
    }

    fn draw_body(&self, frame: &mut Frame, area: Rect, pairs: &HashMap<String, String>) -> () {
        let mut list_items: Vec<ListItem> = Vec::<ListItem>::new();
        let outer_block: Block = Block::bordered().padding(Padding::horizontal(2));

        for (_, (key, value)) in pairs.iter().enumerate() {
            let content: String = format!("{: <25} : {}", key, value);
            let styled_span: Span = Span::styled(content, Style::default().fg(Color::Yellow));

            list_items.push(ListItem::new(Line::from(styled_span)));
        }

        if list_items.len() > 1 {
            frame.render_widget(List::new(list_items).block(outer_block), area);
        } else {
            let no_items_paragraph = Paragraph::new("No JSON Data")
                .style(Style::default().fg(Color::Gray))
                .centered()
                .block(outer_block);

            frame.render_widget(no_items_paragraph, area);
        }
    }

    fn draw_footer(&self, frame: &mut Frame, area: Rect) -> () {
        let screen_name: Span = Span::from("Main Screen").fg(Color::Green);
        let divider: Span = Span::styled(" | ", Style::default().fg(Color::White));
        let current_editing: Span = Span::styled(
            "Not Editing Anything",
            Style::default().fg(Color::DarkGray)
        );

        let footer_text: Vec<Span> = vec![screen_name, divider, current_editing];

        let mode_footer: Paragraph = Paragraph::new(Line::from(footer_text)).block(
            Block::default().borders(Borders::ALL).padding(Padding::horizontal(2))
        );

        let current_keys_hint_text: Span = Span::styled(
            "(Q) to quit / (E) to edit the current JSON",
            Style::default().fg(Color::Red)
        );

        let current_keys_hint: Paragraph = Paragraph::new(Line::from(current_keys_hint_text)).block(
            Block::bordered().padding(Padding::horizontal(2))
        );

        let footer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        frame.render_widget(mode_footer, footer_layout[0]);
        frame.render_widget(current_keys_hint, footer_layout[1]);
    }
}

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
    fn ui(&mut self, app: &mut App, frame: &mut Frame) -> () {
        let layout: Rc<[Rect]> = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)])
            .split(frame.area());

        self.draw_header(frame, layout[0]);
        self.draw_body(frame, layout[1], &app.pairs);
        self.draw_footer(frame, layout[2]);
    }
}
