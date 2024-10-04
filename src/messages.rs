use std::path::PathBuf;

use crate::ProcessResult;

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

    TimeoutChanged(u64),
    TargetNameChanged(String),

    StartProcess,
    DoneProcess,
    DoneEraseProcess(Result<ProcessResult, String>),
    DoneFlashProcess(Result<ProcessResult, String>),
    DoneUnlockProcess(Result<ProcessResult, String>),
}
