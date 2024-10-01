use std::{path::PathBuf, str::FromStr};

use iced::{
    alignment::Horizontal,
    widget::{
        button, center, column, container, opaque, row, stack, text, text_editor, text_input,
    },
    Element, Font, Length, Task, Theme,
};
use iced_aw::{grid, grid_row, number_input};

use crate::{
    log_widget::{LogType, LogWidget},
    messages::Message,
    utils,
};

const TIMEOUT_MIN: u32 = 100;
const TIMEOUT_MAX: u32 = 10000;

pub struct EasyDapLink {
    theme: Theme,
    is_readonly: bool,
    bootloader_path: PathBuf,
    firmware_path: PathBuf,
    user_file_path: PathBuf,
    target_waiting_time: u32,
    target_name: String,
    log_widget: LogWidget,
}

impl EasyDapLink {
    pub fn title(&self) -> String {
        "Easy Flash DAPLink".to_owned()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BrowseBootloader => {
                self.is_readonly = true;

                return Task::perform(
                    utils::select_file(
                        self.bootloader_path.clone(),
                        "Select Bootloader file",
                        false,
                    ),
                    Message::SelectBootloader,
                );
            }
            Message::BrowseFirmware => {
                self.is_readonly = true;

                return Task::perform(
                    utils::select_file(self.firmware_path.clone(), "Select Firmware file", false),
                    Message::SelectFirmware,
                );
            }
            Message::BrowseUserFile => {
                self.is_readonly = true;
                return Task::perform(
                    utils::select_file(
                        self.user_file_path.clone(),
                        "Select user program file",
                        true,
                    ),
                    Message::SelectUserFile,
                );
            }

            Message::SelectBootloader(p) => {
                match p {
                    Some(p) => self.bootloader_path = p,
                    None => (),
                };

                self.is_readonly = false;
            }

            Message::SelectFirmware(p) => {
                match p {
                    Some(p) => self.firmware_path = p,
                    None => (),
                };
                self.is_readonly = false;
            }

            Message::SelectUserFile(p) => {
                match p {
                    Some(p) => self.user_file_path = p,
                    None => (),
                };
                self.is_readonly = false;
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

            Message::StartProcess => {
                self.validate_fields();
            }
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
        .row_spacing(8)
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
        .row_spacing(8)
        .column_widths(&[Length::Shrink, Length::Fill])
        .padding(8);

        let start_button = button(
            text("Start 🚀")
                .shaping(text::Shaping::Advanced)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .width(Length::Fill)
        .on_press(Message::StartProcess);

        let log_view = container(self.log_widget.view())
            .height(Length::Fill)
            .width(Length::Fill);

        let final_view = if self.is_readonly {
            column![
                stack![
                    column![grid_files, grid_settings, start_button].spacing(16),
                    opaque(center(text("")).style(|theme: &Theme| {
                        let mut bg = theme.palette().background;
                        bg.a = 0.8;
                        container::Style {
                            background: Some(bg.into()),
                            ..container::Style::default()
                        }
                    }))
                ],
                log_view
            ]
        } else {
            column![grid_files, grid_settings, start_button, log_view]
        };

        final_view.spacing(16).padding(8).into()
    }

    fn validate_fields(&mut self) -> bool {
        if !self.bootloader_path.exists() {
            self.log_widget.push(LogType::Error(
                "Invalide bootloader file (no such file or directory)".to_owned(),
            ));
            return false;
        }

        if !self.firmware_path.exists() {
            self.log_widget.push(LogType::Error(
                "Invalide firmware file (no such file or directory)".to_owned(),
            ));
            return false;
        }

        if !self.bootloader_path.exists() && !self.user_file_path.to_str().unwrap().is_empty() {
            self.log_widget.push(LogType::Warning(
                "Invalide user file (no such file or directory).".to_owned(),
            ));
        }

        true
    }
}

impl Default for EasyDapLink {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            is_readonly: false,
            bootloader_path: PathBuf::default(),
            firmware_path: PathBuf::default(),
            user_file_path: PathBuf::default(),
            target_waiting_time: 1000,
            target_name: String::default(),
            log_widget: LogWidget::default(),
        }
    }
}
