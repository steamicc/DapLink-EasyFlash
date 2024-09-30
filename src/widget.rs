use std::{path::PathBuf, str::FromStr};

use iced::{
    widget::{button, column, row, text_input},
    Element, Length, Task,
};
use iced_aw::{grid, grid_row, number_input};
use rfd::AsyncFileDialog;

use crate::messages::Message;

const TIMEOUT_MIN: u32 = 100;
const TIMEOUT_MAX: u32 = 10000;

pub struct EasyDapLink {
    is_file_dialog_open: bool,
    bootloader_path: PathBuf,
    firmware_path: PathBuf,
    user_file_path: PathBuf,
    target_waiting_time: u32,
    target_name: String,
}

impl EasyDapLink {
    pub fn title(&self) -> String {
        "Easy Flash DAPLink".to_owned()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BrowseBootloader => {
                if !self.is_file_dialog_open {
                    self.is_file_dialog_open = true;

                    return Task::perform(
                        Self::select_file(
                            self.bootloader_path.clone(),
                            "Select Bootloader file",
                            false,
                        ),
                        Message::SelectBootloader,
                    );
                }
            }
            Message::BrowseFirmware => {
                if !self.is_file_dialog_open {
                    self.is_file_dialog_open = true;

                    return Task::perform(
                        Self::select_file(
                            self.firmware_path.clone(),
                            "Select Firmware file",
                            false,
                        ),
                        Message::SelectFirmware,
                    );
                }
            }
            Message::BrowseUserFile => {
                if !self.is_file_dialog_open {
                    self.is_file_dialog_open = true;

                    return Task::perform(
                        Self::select_file(
                            self.user_file_path.clone(),
                            "Select user program file",
                            true,
                        ),
                        Message::SelectUserFile,
                    );
                }
            }

            Message::SelectBootloader(p) => {
                match p {
                    Some(p) => self.bootloader_path = p,
                    None => (),
                };

                self.is_file_dialog_open = false;
            }

            Message::SelectFirmware(p) => {
                match p {
                    Some(p) => self.firmware_path = p,
                    None => (),
                };
                self.is_file_dialog_open = false;
            }

            Message::SelectUserFile(p) => {
                match p {
                    Some(p) => self.user_file_path = p,
                    None => (),
                };
                self.is_file_dialog_open = false;
            }

            Message::InputBootloaderPath(s) => {
                self.bootloader_path = PathBuf::from_str(&s).unwrap()
            }
            Message::InputFirmwarePath(s) => self.firmware_path = PathBuf::from_str(&s).unwrap(),
            Message::InputUserFilePath(s) => self.user_file_path = PathBuf::from_str(&s).unwrap(),

            Message::TimeoutChanged(v) => {
                self.target_waiting_time = v.clamp(TIMEOUT_MIN, TIMEOUT_MAX)
            }

            Message::TargetNameChanged(s) => self.target_name = s,
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        let grid_files = grid!(
            grid_row!(
                "Bootloader file",
                row![
                    text_input(
                        "Bootloader",
                        self.bootloader_path.to_str().unwrap_or_default()
                    )
                    .on_input(Message::InputBootloaderPath)
                    .width(Length::Fill),
                    button("...").on_press(Message::BrowseBootloader)
                ]
                .spacing(8)
            ),
            grid_row!(
                "Firmware file",
                row![
                    text_input("Firmware", self.firmware_path.to_str().unwrap_or_default())
                        .on_input(Message::InputFirmwarePath)
                        .width(Length::Fill),
                    button("...").on_press(Message::BrowseFirmware)
                ]
                .spacing(8)
            ),
            grid_row!(
                "(Optionnal) User program",
                row![
                    text_input(
                        "User program",
                        self.user_file_path.to_str().unwrap_or_default()
                    )
                    .on_input(Message::InputUserFilePath)
                    .width(Length::Fill),
                    button("...").on_press(Message::BrowseUserFile)
                ]
                .spacing(8)
            ),
        )
        .width(Length::Fill)
        .column_spacing(8)
        .row_spacing(16)
        .column_widths(&[Length::Shrink, Length::Fill])
        .padding(8);

        let grid_settings = grid!(
            grid_row!(
                "Target mount name",
                text_input("STeaMi, DIS_L4IOT, ...", &self.target_name)
                    .on_input(Message::TargetNameChanged)
                    .width(200),
            ),
            grid_row!(
                "Timeout (ms) for mount points",
                number_input(
                    self.target_waiting_time,
                    TIMEOUT_MIN..=TIMEOUT_MAX,
                    Message::TimeoutChanged
                )
                .step(100)
                .width(Length::Fill)
            ),
        )
        .width(Length::Fill)
        .column_spacing(8)
        .row_spacing(16)
        .column_widths(&[Length::Shrink, Length::Fill])
        .padding(8);

        column![grid_files, grid_settings]
            .spacing(32)
            .padding(8)
            .into()
    }

    async fn select_file(current: PathBuf, title: &str, allow_hex: bool) -> Option<PathBuf> {
        let mut dialog = AsyncFileDialog::new()
            .set_title(title)
            .add_filter("Binary file (*.bin)", &["bin", "BIN"]);

        if allow_hex {
            dialog = dialog.add_filter("Hex file (*.hex)", &["hex", "HEX"]);
        }

        dialog = dialog.add_filter("All file", &["*"]);

        if current.exists() {
            if current.is_dir() {
                dialog = dialog.set_directory(current);
            } else {
                match current.parent() {
                    Some(p) => dialog = dialog.set_directory(p),
                    None => (),
                };
            }
        }

        dialog.pick_file().await.map(|h| h.path().to_path_buf())
    }
}

impl Default for EasyDapLink {
    fn default() -> Self {
        Self {
            is_file_dialog_open: false,
            bootloader_path: PathBuf::default(),
            firmware_path: PathBuf::default(),
            user_file_path: PathBuf::default(),
            target_waiting_time: 1000,
            target_name: "STeaMi".to_owned(),
        }
    }
}
