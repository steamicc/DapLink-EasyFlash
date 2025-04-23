use std::{fs, path::PathBuf, str::FromStr, time::Duration};

use iced::{
    alignment::Horizontal,
    widget::{button, center, column, container, opaque, row, stack, text, text_input},
    Element, Length, Task, Theme,
};
use iced_aw::{grid, grid_row, number_input};
use serde::{Deserialize, Serialize};

use crate::{dirs, disk_tool, log_entries::LogType, open_ocd_task, utils};

use super::{
    log_widget::LogWidget,
    messages::{Message, TabDaplinkMessage},
};

const MAINTENANCE_DISK_NAME: &str = "MAINTENANCE";
const TIMEOUT_MIN: u64 = 1;
const TIMEOUT_MAX: u64 = 30;

fn default_target_waiting_time() -> u64 {
    10
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabDaplink {
    #[serde(skip)]
    is_readonly: bool,
    bootloader_path: PathBuf,
    firmware_path: PathBuf,
    user_file_path: PathBuf,
    #[serde(default = "default_target_waiting_time")]
    target_waiting_time: u64,
    target_name: String,
    #[serde(skip)]
    log_widget: LogWidget,
}

impl TabDaplink {
    pub fn update(&mut self, message: TabDaplinkMessage) -> Task<Message> {
        match message {
            TabDaplinkMessage::BrowseBootloader => {
                self.is_readonly = true;

                return Task::perform(
                    utils::select_file(
                        self.bootloader_path.clone(),
                        "Select Bootloader file",
                        false,
                    ),
                    |x| Message::DapLink(TabDaplinkMessage::SelectBootloader(x)),
                );
            }
            TabDaplinkMessage::BrowseFirmware => {
                self.is_readonly = true;

                return Task::perform(
                    utils::select_file(self.firmware_path.clone(), "Select Firmware file", false),
                    |x| Message::DapLink(TabDaplinkMessage::SelectFirmware(x)),
                );
            }
            TabDaplinkMessage::BrowseUserFile => {
                self.is_readonly = true;
                return Task::perform(
                    utils::select_file(
                        self.user_file_path.clone(),
                        "Select user program file",
                        true,
                    ),
                    |x| Message::DapLink(TabDaplinkMessage::SelectUserFile(x)),
                );
            }

            TabDaplinkMessage::SelectBootloader(p) => {
                match p {
                    Some(p) => self.bootloader_path = p,
                    None => (),
                };

                self.is_readonly = false;
            }

            TabDaplinkMessage::SelectFirmware(p) => {
                match p {
                    Some(p) => self.firmware_path = p,
                    None => (),
                };
                self.is_readonly = false;
            }

            TabDaplinkMessage::SelectUserFile(p) => {
                match p {
                    Some(p) => self.user_file_path = p,
                    None => (),
                };
                self.is_readonly = false;
            }

            TabDaplinkMessage::InputBootloaderPath(s) => {
                self.bootloader_path = PathBuf::from_str(&s).unwrap()
            }
            TabDaplinkMessage::InputFirmwarePath(s) => {
                self.firmware_path = PathBuf::from_str(&s).unwrap()
            }
            TabDaplinkMessage::InputUserFilePath(s) => {
                self.user_file_path = PathBuf::from_str(&s).unwrap()
            }

            TabDaplinkMessage::TimeoutChanged(v) => {
                self.target_waiting_time = v.clamp(TIMEOUT_MIN, TIMEOUT_MAX);
            }

            TabDaplinkMessage::TargetNameChanged(s) => self.target_name = s,

            TabDaplinkMessage::StartProcess => {
                if !self.validate_fields() {
                    return Task::none();
                }

                match open_ocd_task::is_installed() {
                    Ok(is_install) => {
                        if !is_install {
                            self.log_widget
                                .push(LogType::Error("OpenOCD is not found".into()));
                            return Task::none();
                        }
                    }
                    Err(e) => {
                        self.log_widget.push(LogType::Error(format!(
                            "Failed to test openocd installation: {e}"
                        )));
                        return Task::none();
                    }
                };

                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                self.log_widget.push(LogType::Info("Unlock target".into()));
                self.is_readonly = true;
                return Task::perform(open_ocd_task::unlock_target(), |x| {
                    Message::DapLink(TabDaplinkMessage::DoneUnlockProcess(x))
                });
            }

            TabDaplinkMessage::DoneProcess => {
                self.is_readonly = false;
            }

            TabDaplinkMessage::DoneUnlockProcess(result) => {
                if result.is_err() {
                    self.log_widget.push(LogType::Error(format!(
                        "Failed to run unlock process. Error: {}",
                        result.err().unwrap()
                    )));
                } else {
                    let result = result.unwrap();

                    self.log_widget.from_log_entries(&result.log);

                    match result.code {
                        Some(code) => {
                            if code == 0 {
                                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                                self.log_widget.push(LogType::Info("Erase target".into()));
                                return Task::perform(open_ocd_task::erase_target(), |x| {
                                    Message::DapLink(TabDaplinkMessage::DoneEraseProcess(x))
                                });
                            } else {
                                self.log_widget
                                    .push(LogType::Warning(format!("Exit code: {}", code)));
                            }
                        }
                        None => self
                            .log_widget
                            .push(LogType::Warning("Process terminated by signal.".into())),
                    }
                }
                return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
            }
            TabDaplinkMessage::DoneEraseProcess(result) => {
                if result.is_err() {
                    self.log_widget.push(LogType::Error(format!(
                        "Failed to run erase process. Error: {}",
                        result.err().unwrap()
                    )));
                } else {
                    let result = result.unwrap();

                    self.log_widget.from_log_entries(&result.log);

                    match result.code {
                        Some(code) => {
                            if code == 0 {
                                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                                self.log_widget
                                    .push(LogType::Info("Flash bootloader".into()));

                                return Task::perform(
                                    open_ocd_task::flash_target(self.bootloader_path.clone()),
                                    |x| Message::DapLink(TabDaplinkMessage::DoneFlashProcess(x)),
                                );
                            } else {
                                self.log_widget
                                    .push(LogType::Warning(format!("Exit code: {}", code)));
                            }
                        }
                        None => self
                            .log_widget
                            .push(LogType::Warning("Process terminated by signal.".into())),
                    }
                }
                return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
            }
            TabDaplinkMessage::DoneFlashProcess(result) => {
                if result.is_err() {
                    self.log_widget.push(LogType::Error(format!(
                        "Failed to run erase process. Error: {}",
                        result.err().unwrap()
                    )));
                } else {
                    let result = result.unwrap();

                    self.log_widget.from_log_entries(&result.log);

                    match result.code {
                        Some(code) => {
                            if code == 0 {
                                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                                self.log_widget.push(LogType::Info(format!(
                                    "Wait for '{MAINTENANCE_DISK_NAME}' drive"
                                )));
                                return Task::perform(
                                    disk_tool::wait_for_drive(
                                        MAINTENANCE_DISK_NAME.into(),
                                        Duration::from_secs(self.target_waiting_time),
                                    ),
                                    |x| {
                                        Message::DapLink(
                                            TabDaplinkMessage::DoneWaitMaintenanceDisk(x),
                                        )
                                    },
                                );
                            } else {
                                self.log_widget
                                    .push(LogType::Warning(format!("Exit code: {}", code)));
                            }
                        }
                        None => self
                            .log_widget
                            .push(LogType::Warning("Process terminated by signal.".into())),
                    }
                }
                return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
            }

            TabDaplinkMessage::DoneWaitMaintenanceDisk(is_found) => {
                if !is_found {
                    self.log_widget.push(LogType::Error(format!(
                        "TIMEOUT : The device '{MAINTENANCE_DISK_NAME}' was not found."
                    )));
                    return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
                }

                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                self.log_widget.push(LogType::Info(format!(
                    "Copy firmware to {MAINTENANCE_DISK_NAME}"
                )));
                return Task::perform(
                    disk_tool::copy_file_to_disk(
                        MAINTENANCE_DISK_NAME.into(),
                        self.firmware_path.clone(),
                    ),
                    |x| Message::DapLink(TabDaplinkMessage::DoneCopyFirmware(x)),
                );
            }
            TabDaplinkMessage::DoneCopyFirmware(result) => {
                match result {
                    Ok(_) => {
                        if self.user_file_path.exists() && self.user_file_path.is_file() {
                            self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                            self.log_widget.push(LogType::Info(format!(
                                "Wait for '{}' drive",
                                self.target_name
                            )));
                            return Task::perform(
                                disk_tool::wait_for_drive(
                                    self.target_name.clone(),
                                    Duration::from_secs(self.target_waiting_time),
                                ),
                                |x| Message::DapLink(TabDaplinkMessage::DoneWaitingDeviceDisk(x)),
                            );
                        } else {
                            self.log_widget
                                .push(LogType::Warning("No user file. Skip.".into()));
                        }
                    }
                    Err(e) => self
                        .log_widget
                        .push(LogType::Error(format!("Copy failed ({e})"))),
                }
                return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
            }

            TabDaplinkMessage::DoneWaitingDeviceDisk(is_found) => {
                if !is_found {
                    self.log_widget.push(LogType::Error(format!(
                        "TIMEOUT : The device '{}' was not found.",
                        self.target_name
                    )));
                    return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
                }

                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                self.log_widget.push(LogType::Info(format!(
                    "Copy firmware to {}",
                    self.target_name
                )));
                return Task::perform(
                    disk_tool::copy_file_to_disk(
                        self.target_name.clone(),
                        self.user_file_path.clone(),
                    ),
                    |x| Message::DapLink(TabDaplinkMessage::DoneCopyUserfile(x)),
                );
            }

            TabDaplinkMessage::DoneCopyUserfile(result) => {
                match result {
                    Ok(_) => (),
                    Err(e) => self
                        .log_widget
                        .push(LogType::Error(format!("Copy failed ({e})"))),
                }
                self.log_widget.push(LogType::InfoNoPrefix("\n\n".into()));
                return Task::done(Message::DapLink(TabDaplinkMessage::DoneProcess));
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
                    .on_input(|s| Message::DapLink(TabDaplinkMessage::InputBootloaderPath(s)))
                    .width(Length::Fill),
                    button("...").on_press(Message::DapLink(TabDaplinkMessage::BrowseBootloader))
                ]
                .spacing(8)
            ),
            grid_row!(
                "Firmware file",
                row![
                    text_input("Firmware", self.firmware_path.to_str().unwrap_or_default())
                        .on_input(|s| Message::DapLink(TabDaplinkMessage::InputFirmwarePath(s)))
                        .width(Length::Fill),
                    button("...").on_press(Message::DapLink(TabDaplinkMessage::BrowseFirmware))
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
                    .on_input(|s| Message::DapLink(TabDaplinkMessage::InputUserFilePath(s)))
                    .width(Length::Fill),
                    button("...").on_press(Message::DapLink(TabDaplinkMessage::BrowseUserFile))
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
                    .on_input(|s| Message::DapLink(TabDaplinkMessage::TargetNameChanged(s)))
                    .width(200),
            ),
            grid_row!(
                "Timeout (s) for mount points",
                number_input(self.target_waiting_time, TIMEOUT_MIN..=TIMEOUT_MAX, |x| {
                    Message::DapLink(TabDaplinkMessage::TimeoutChanged(x))
                })
                .step(1)
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
        .on_press(Message::DapLink(TabDaplinkMessage::StartProcess));

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

    // TODO Improve fields loading/saving
    fn load_fields() -> Option<Self> {
        match dirs::get_settings_dir() {
            Ok(settings_dir) => {
                let fields_file = settings_dir.join("fields.json");
                match fs::read_to_string(fields_file) {
                    Ok(str) => match serde_json::from_str(&str) {
                        Ok(object) => return Some(object),
                        Err(e) => eprintln!("Failed to load fields ({e})"),
                    },
                    Err(e) => eprintln!("Failed to read fields file ({e})"),
                }
            }
            Err(e) => eprintln!("Failed to get settings dirs (Error: {e}"),
        };

        None
    }
}

impl Default for TabDaplink {
    fn default() -> Self {
        let mut object = Self {
            is_readonly: false,
            bootloader_path: PathBuf::default(),
            firmware_path: PathBuf::default(),
            user_file_path: PathBuf::default(),
            target_waiting_time: 10,
            target_name: String::default(),
            log_widget: LogWidget::default(),
        };

        match Self::load_fields() {
            Some(saved) => {
                object.bootloader_path = saved.bootloader_path;
                object.firmware_path = saved.firmware_path;
                object.user_file_path = saved.user_file_path;
                object.target_name = saved.target_name;
                object.target_waiting_time = saved.target_waiting_time;
            }
            None => (),
        }

        object
    }
}
