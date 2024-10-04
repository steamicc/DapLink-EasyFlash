use std::path::PathBuf;

use crate::log_entries::LogEntries;

#[derive(Debug, Clone)]
pub enum Message {
    BrowseBootloader,
    BrowseFirmware,
    BrowseUserFile,
    SelectBootloader(Option<PathBuf>),
    SelectFirmware(Option<PathBuf>),
    SelectUserFile(Option<PathBuf>),

    InputBootloaderPath(String),
    InputFirmwarePath(String),
    InputUserFilePath(String),

    TimeoutChanged(u32),
    TargetNameChanged(String),

    StartProcess,
    DoneProcess,
    DoneEraseProcess(Result<(Option<i32>, LogEntries), String>),
    DoneFlashProcess(Result<(Option<i32>, LogEntries), String>),
    DoneUnlockProcess(Result<(Option<i32>, LogEntries), String>),
}
