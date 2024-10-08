use std::process::exit;

use iced::{window, Settings, Size};
use log_entries::LogEntries;
use main_widget::EasyDapLink;

mod dirs;
mod disk_tool;
mod log_entries;
mod log_widget;
mod main_widget;
mod messages;
mod open_ocd_task;
mod utils;

#[derive(Debug, Clone)]
struct ProcessResult {
    pub code: Option<i32>,
    pub log: LogEntries,
}

fn main() -> iced::Result {
    match open_ocd_task::create_script_file() {
        Ok(_) => println!("Script created/updated"),
        Err(e) => {
            eprintln!("Failed to create/update scripts ({e})");
            exit(100);
        }
    };

    iced::application(EasyDapLink::title, EasyDapLink::update, EasyDapLink::view)
        .theme(EasyDapLink::theme)
        .settings(Settings::default())
        .font(iced_fonts::REQUIRED_FONT_BYTES)
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
        .subscription(EasyDapLink::application_subscription)
        .exit_on_close_request(false)
        .run()
}
