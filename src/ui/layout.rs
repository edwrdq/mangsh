use ratatui::layout::{Layout, Constraint, Rect};
use std::rc::Rc;

pub fn split_main(area: Rect) -> Rc<[Rect]> {
    Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
    ]).split(area)
}
