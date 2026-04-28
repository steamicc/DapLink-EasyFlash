//! Tiny helpers replacing the `iced_aw` 0.11 `grid!` / `grid_row!` macros that
//! were dropped in 0.14. The layout is a fixed-width label column followed by
//! a `Length::Fill` content column — same visual as before.

use iced::{
    alignment::Vertical,
    widget::{column, row, text, Column, Row},
    Element, Length,
};

const LABEL_WIDTH: f32 = 200.0;
const ROW_SPACING: f32 = 8.0;

/// One label + content row with the standard label width.
pub fn form_row<'a, Message: 'a>(
    label: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Row<'a, Message> {
    row![
        text(label)
            .width(Length::Fixed(LABEL_WIDTH))
            .align_y(Vertical::Center),
        content.into(),
    ]
    .spacing(ROW_SPACING)
    .align_y(Vertical::Center)
}

/// Stacks form rows into a column with the standard inter-row spacing.
pub fn form<'a, Message: 'a>(rows: Vec<Row<'a, Message>>) -> Column<'a, Message> {
    let mut col = column![].spacing(ROW_SPACING).padding(8);
    for r in rows {
        col = col.push(r);
    }
    col
}
