use iced::{
    widget::{column, scrollable, text, Column, Text},
    Color, Element, Font, Length,
};

use crate::messages::Message;

pub enum LogType {
    Info(String),
    Warning(String),
    Error(String),
}

#[derive(Default)]
pub struct LogWidget {
    entries: Vec<LogType>,
}

impl LogWidget {
    pub fn push(&mut self, entry: LogType) {
        self.entries.push(entry);
    }

    pub fn view(&self) -> Element<Message> {
        let iter: Vec<Element<Message>> = self
            .entries
            .iter()
            .map(|entry| match entry {
                LogType::Info(s) => Text::new(format!("[INFO] {s}"))
                    .color(Color::from_rgb8(0, 0, 0))
                    .font(Font::MONOSPACE)
                    .into(),
                LogType::Warning(s) => Text::new(format!("[WARN] {s}"))
                    .color(Color::from_rgb8(0xAB, 0x69, 0))
                    .font(Font::MONOSPACE)
                    .into(),
                LogType::Error(s) => Text::new(format!("[ERR] {s}"))
                    .color(Color::from_rgb8(0xAA, 0, 0))
                    .font(Font::MONOSPACE)
                    .into(),
            })
            .collect();

        scrollable(Column::with_children(iter))
            .anchor_bottom()
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
