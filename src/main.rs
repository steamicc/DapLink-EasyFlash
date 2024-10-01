use iced::{window, Settings, Size};
use main_widget::EasyDapLink;

mod disk_tool;
mod log_widget;
mod main_widget;
mod messages;
mod open_ocd_task;
mod utils;

fn main() -> iced::Result {
    iced::application(EasyDapLink::title, EasyDapLink::update, EasyDapLink::view)
        .theme(EasyDapLink::theme)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .settings(Settings::default())
        .window(window::Settings {
            size: Size {
                width: 550.0,
                height: 700.0,
            },
            min_size: Some(Size {
                width: 480.0,
                height: 460.0,
            }),
            ..Default::default()
        })
        .run()
}
