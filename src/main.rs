use std::process::exit;

use iced::{window, Settings, Size};
use log_entries::LogEntries;
use ui::main_window::MainWindow;

mod dirs;
mod disk_tool;
mod log_entries;
mod open_ocd_task;
mod stackfile_config;
mod utils;

mod ui;

#[derive(Debug, Clone)]
struct ProcessResult {
    pub code: Option<i32>,
    pub log: LogEntries,
}

fn main() -> iced::Result {
    match std::env::current_exe() {
        Ok(mut path) => {
            path.pop();
            dirs::set_exe_dir(path);
        }
        Err(e) => {
            eprintln!("Failed to create/update scripts ({e})");
            exit(100);
        }
    }

    iced::application(MainWindow::title, MainWindow::update, MainWindow::view)
        .theme(MainWindow::theme)
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
        .subscription(MainWindow::application_subscription)
        .exit_on_close_request(false)
        .run()
}
