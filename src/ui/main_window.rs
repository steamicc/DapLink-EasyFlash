use std::{fs, path::PathBuf, str::FromStr, time::Duration};

use iced::{
    advanced::graphics::futures::event,
    alignment::Horizontal,
    widget::{button, center, column, container, opaque, row, stack, text, text_input},
    Element, Event, Length, Subscription, Task, Theme,
};
use iced_aw::{grid, grid_row, number_input};
use serde::{Deserialize, Serialize};

use crate::{dirs, disk_tool, log_entries::LogType, open_ocd_task, utils};

use super::{log_widget::LogWidget, messages::Message, tab_daplink::TabDaplink};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MainWindow {
    #[serde(skip)]
    theme: Theme,
    tab_daplink: TabDaplink,
}

impl MainWindow {
    pub fn title(&self) -> String {
        "Easy Flash DAPLink".to_owned()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ApplicationEvent(event) => match event {
                Event::Keyboard(_) | Event::Mouse(_) | Event::Touch(_) => Task::none(),
                Event::Window(event) => match event {
                    iced::window::Event::CloseRequested => {
                        match dirs::get_settings_dir() {
                            Ok(settings_dir) => {
                                let fields_file = settings_dir.join("fields.json");
                                match fs::write(
                                    fields_file,
                                    serde_json::to_string_pretty(&self).unwrap_or("{}".into()),
                                ) {
                                    Ok(_) => println!("Fields succesfully saved"),
                                    Err(e) => eprintln!("Failed to save fields ({e})"),
                                }
                            }
                            Err(e) => eprintln!("Failed to get settings dirs (Error: {e}"),
                        };
                        return iced::window::get_latest().and_then(iced::window::close);
                    }
                    _ => Task::none(),
                },
            },
            Message::DapLink(tab_daplink_message) => self.tab_daplink.update(tab_daplink_message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        self.tab_daplink.view()
    }

    pub fn application_subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::ApplicationEvent)
    }
}
