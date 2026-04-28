use std::fs;

use iced::{
    advanced::graphics::futures::event, widget::column, Element, Event, Subscription, Task, Theme,
};
use iced_aw::{TabBar, TabLabel};
use serde::{Deserialize, Serialize};

use crate::dirs;

use super::{messages::Message, tab_daplink::TabDaplink, tab_wireless_stack::TabWirelessStack};

const DAPLINK_TAB: u16 = 0;
const WIRELESS_STACK_TAB: u16 = 1;
const SETTINGS_FILE: &str = "fields.json";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MainWindow {
    #[serde(skip)]
    theme: Theme,
    #[serde(skip)]
    active_tab: u16,
    tab_daplink: TabDaplink,
    tab_ws: TabWirelessStack,
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
            Message::DapLink(dp_message) => self.tab_daplink.update(dp_message),
            Message::WirelessStack(ws_message) => self.tab_ws.update(ws_message),

            Message::ApplicationEvent(event) => match event {
                Event::Keyboard(_) | Event::Mouse(_) | Event::Touch(_) => Task::none(),
                Event::Window(event) => match event {
                    iced::window::Event::Opened { .. } => {
                        match dirs::get_settings_dir() {
                            Ok(settings_dir) => {
                                let fields_file = settings_dir.join(SETTINGS_FILE);
                                match fs::read(fields_file) {
                                    Ok(content) => self.load_settings(&content),
                                    Err(e) => eprintln!("Failed to open settings file ({e})"),
                                }
                            }
                            Err(e) => eprintln!("Failed to get settings dirs (Error: {e}"),
                        };
                        self.tab_ws.refresh_serial_ports();
                        return Task::none();
                    }
                    iced::window::Event::CloseRequested => {
                        match dirs::get_settings_dir() {
                            Ok(settings_dir) => {
                                let fields_file = settings_dir.join(SETTINGS_FILE);
                                match fs::write(
                                    fields_file,
                                    serde_json::to_string_pretty(&self).unwrap_or("{}".into()),
                                ) {
                                    Ok(_) => println!("Settings successfully saved"),
                                    Err(e) => eprintln!("Failed to save settings ({e})"),
                                }
                            }
                            Err(e) => eprintln!("Failed to get settings dirs (Error: {e}"),
                        };
                        return iced::window::get_latest().and_then(iced::window::close);
                    }
                    _ => Task::none(),
                },
            },
            Message::TabBarSelected(tab_idx) => {
                self.active_tab = tab_idx;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut col = column![TabBar::new(Message::TabBarSelected)
            .push(DAPLINK_TAB, TabLabel::Text("DapLink".into()))
            .push(WIRELESS_STACK_TAB, TabLabel::Text("Wireless Stack".into()))
            .padding(1)
            .set_active_tab(&self.active_tab)];

        col = match self.active_tab {
            DAPLINK_TAB => col.push(self.tab_daplink.view()),
            WIRELESS_STACK_TAB => col.push(self.tab_ws.view()),
            _ => {
                eprintln!("Invalid selected tab ({})", self.active_tab);
                col
            }
        };

        col.into()
    }

    pub fn application_subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::ApplicationEvent)
    }

    /// Deserialize the on-disk settings, with a fallback to the legacy 0.1.x
    /// schema where `TabDaplink` fields were serialized directly at the top
    /// level and are now mapped onto `tab_daplink`. Successful migrations get
    /// rewritten in the new shape on the next `CloseRequested`.
    fn load_settings(&mut self, content: &[u8]) {
        match serde_json::from_slice::<Self>(content) {
            Ok(obj) => {
                self.tab_daplink = obj.tab_daplink;
                self.tab_ws = obj.tab_ws;
                println!("Settings loaded !");
            }
            Err(new_err) => match serde_json::from_slice::<TabDaplink>(content) {
                Ok(legacy) => {
                    self.tab_daplink = legacy;
                    println!("Legacy settings migrated to the new schema");
                }
                Err(legacy_err) => {
                    eprintln!(
                        "Failed to deserialize settings. New schema error: {new_err}. Legacy schema error: {legacy_err}"
                    );
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEGACY_JSON: &[u8] = br#"{
        "bootloader_path": "/tmp/bl.bin",
        "firmware_path": "/tmp/fw.bin",
        "user_file_path": "/tmp/user.bin",
        "target_waiting_time": 7,
        "target_name": "LEGACY-STEAMI"
    }"#;

    const NEW_JSON: &[u8] = br#"{
        "tab_daplink": {
            "bootloader_path": "/tmp/bl.bin",
            "firmware_path": "/tmp/fw.bin",
            "user_file_path": "/tmp/user.bin",
            "target_waiting_time": 7,
            "target_name": "LEGACY-STEAMI"
        },
        "tab_ws": {
            "fw_selected": "BleHciExt"
        }
    }"#;

    #[test]
    fn loads_new_schema() {
        let mut w = MainWindow::default();
        w.load_settings(NEW_JSON);
        let s = serde_json::to_string(&w).unwrap();
        assert!(s.contains(r#""target_name":"LEGACY-STEAMI""#));
        assert!(s.contains(r#""fw_selected":"BleHciExt""#));
    }

    #[test]
    fn legacy_schema_migrates_and_serializes_in_new_shape() {
        let mut w = MainWindow::default();
        w.load_settings(LEGACY_JSON);

        // Re-serialize the way the CloseRequested handler does.
        let s = serde_json::to_string(&w).unwrap();

        // New shape: top-level keys for both tabs.
        assert!(s.contains(r#""tab_daplink""#), "missing tab_daplink in: {s}");
        assert!(s.contains(r#""tab_ws""#), "missing tab_ws in: {s}");

        // Legacy data preserved under tab_daplink.
        assert!(s.contains(r#""bootloader_path":"/tmp/bl.bin""#));
        assert!(s.contains(r#""firmware_path":"/tmp/fw.bin""#));
        assert!(s.contains(r#""user_file_path":"/tmp/user.bin""#));
        assert!(s.contains(r#""target_waiting_time":7"#));
        assert!(s.contains(r#""target_name":"LEGACY-STEAMI""#));
    }

    #[test]
    fn malformed_json_keeps_defaults() {
        let mut w = MainWindow::default();
        w.load_settings(b"{ this is not json");
        // No assertion on stderr — load_settings reports both errors via eprintln,
        // exercised manually. The state must remain at defaults: serializing
        // shouldn't carry any of the malformed input.
        let s = serde_json::to_string(&w).unwrap();
        assert!(!s.contains("LEGACY-STEAMI"));
    }

    #[test]
    fn empty_content_keeps_defaults() {
        let mut w = MainWindow::default();
        w.load_settings(b"");
        let s = serde_json::to_string(&w).unwrap();
        assert!(s.contains(r#""tab_daplink""#));
        assert!(s.contains(r#""tab_ws""#));
    }
}
