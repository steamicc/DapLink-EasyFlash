use std::path::PathBuf;

use iced::Event;

use crate::ProcessResult;

use super::tab_wireless_stack::{SerialPort, WirelessStackFile};

#[derive(Debug, Clone)]
pub enum Message {
    DapLink(TabDaplinkMessage),
    WirelessStack(TabWirelessStackMessage),

    TabBarSelected(u16),
    ApplicationEvent(Event),
}

#[derive(Debug, Clone)]
pub enum TabDaplinkMessage {
    BrowseBootloader,
    BrowseFirmware,
    BrowseUserFile,
    SelectBootloader(Option<PathBuf>),
    SelectFirmware(Option<PathBuf>),
    SelectUserFile(Option<PathBuf>),

    InputBootloaderPath(String),
    InputFirmwarePath(String),
    InputUserFilePath(String),

    TimeoutChanged(u64),
    TargetNameChanged(String),

    StartProcess,
    DoneProcess,
    DoneEraseProcess(Result<ProcessResult, String>),
    DoneFlashProcess(Result<ProcessResult, String>),
    DoneUnlockProcess(Result<ProcessResult, String>),

    DoneWaitMaintenanceDisk(bool),
    DoneCopyFirmware(Result<(), String>),
    DoneWaitingDeviceDisk(bool),
    DoneCopyUserfile(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum TabWirelessStackMessage {
    StackSelected(WirelessStackFile),
    SerialSelected(SerialPort),
    SerialRefresh,
    StartProcess,
}
