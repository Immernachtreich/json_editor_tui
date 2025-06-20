use ratatui::{ layout::{ Constraint, Direction, Layout, Rect }, Frame };
use std::rc::Rc;

/// Centers a rectangle within another, given as a percentage of the parent's size
///
/// Given a percentage width and height, this will return a rectangle that is
/// centered within the given rectangle, with the given percentage of that
/// rectangle's size.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the rectangle into 3 vertical pieces
    let popup_layout: Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Cut the middle vertical piece into three different pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn ui(frame: &mut Frame) {}
