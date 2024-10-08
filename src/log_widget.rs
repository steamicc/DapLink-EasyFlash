use iced::{
    widget::{scrollable, Column, Text},
    Color, Element, Font, Length, Pixels,
};

use crate::{
    log_entries::{LogEntries, LogType},
    messages::Message,
};

const TEXT_SIZE: u16 = 12;

#[derive(Default, Debug)]
pub struct LogWidget {
    log: LogEntries,
}

impl LogWidget {
    pub fn push(&mut self, entry: LogType) {
        self.log.push(entry);
    }

    pub fn from_log_entries(&mut self, log: &LogEntries) {
        while let Some(entry) = log.pop() {
            self.log.push(entry);
        }
    }

    pub fn view(&self) -> Element<Message> {
        let iter: Vec<Element<Message>> = self
            .log
            .as_deque()
            .iter()
            .map(|entry| {
                let entry: Text = match entry {
                    LogType::Info(s) => Text::new(format!("[INFO] {s}")),
                    LogType::InfoNoPrefix(s) => Text::new(s.clone()),
                    LogType::Warning(s) => {
                        Text::new(format!("[WARN] {s}")).color(Color::from_rgb8(0xAB, 0x69, 0))
                    }
                    LogType::Error(s) => {
                        Text::new(format!("[ERR] {s}")).color(Color::from_rgb8(0xAA, 0, 0))
                    }
                };
                entry
                    .size(Pixels::from(TEXT_SIZE))
                    .font(Font::MONOSPACE)
                    .into()
            })
            .collect();

        scrollable(Column::with_children(iter))
            .anchor_bottom()
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
