use std::{
    collections::VecDeque,
    fs, i32,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use iced::futures::{channel::mpsc::Sender, SinkExt};

use crate::{
    dirs,
    log_entries::{LogEntries, LogType},
    ui::messages::{TabDaplinkMessage, TabWsMessage, WithLogMessage},
    ProcessResult,
};

pub const UNLOCK_SCRIPT_FILENAME: &str = "f1x-unlock.cfg";
pub const ERASE_SCRIPT_FILENAME: &str = "f1x-erase.cfg";
pub const FLASH_SCRIPT_FILENAME: &str = "f1x-flash.cfg";
pub const WB55_CONFIG: &str = "wb5x.cfg";

pub async fn unlock_target() -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_configs_dir()?;
    let path_script = script_folder.join(UNLOCK_SCRIPT_FILENAME);

    let mut command = Command::new("openocd");
    command.args(&["-f", &format!("{}", path_script.to_str().unwrap())]);

    Ok(run_command_sender::<TabDaplinkMessage>(&mut command, None).await?)
}

pub async fn erase_target() -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_configs_dir()?;
    let path_script = script_folder.join(ERASE_SCRIPT_FILENAME);

    let mut command = Command::new("openocd");
    command.args(&["-f", &format!("{}", path_script.to_str().unwrap())]);

    Ok(run_command_sender::<TabDaplinkMessage>(&mut command, None).await?)
}

pub async fn flash_target(bin_path: PathBuf) -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_configs_dir()?;
    let path_script = script_folder.join(FLASH_SCRIPT_FILENAME);

    if !bin_path.is_file() {
        return Err("The firmware path is not a file.".into());
    }

    match fs::copy(bin_path, dirs::get_tmp_dir()?.join("bootloader")) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to copy bootloader file ({e}")),
    };

    let tmp_dir_string = dirs::get_tmp_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| "Failed to convert tmp_dir to string.")?;

    let mut command = Command::new("openocd");
    command.args(&[
        "-s",
        &tmp_dir_string,
        "-f",
        &format!("{}", path_script.to_str().unwrap()),
    ]);

    Ok(run_command_sender::<TabDaplinkMessage>(&mut command, None).await?)
}

pub fn is_installed() -> Result<bool, String> {
    let child = Command::new("openocd")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    Ok(child.status.success())
}

pub async fn flash_wb55(
    file: &str,
    sender: &mut Sender<TabWsMessage>,
) -> Result<ProcessResult, String> {
    let mut command = Command::new("openocd");
    command.args(&[
        "-f",
        WB55_CONFIG,
        "-c",
        &format!("program {} verify reset", file),
        "-c",
        "reset run",
        "-c",
        "exit",
    ]);

    run_command_sender(&mut command, Some(sender)).await
}

async fn run_command_sender<MSG>(
    cmd: &mut Command,
    mut sender: Option<&mut Sender<MSG>>,
) -> Result<ProcessResult, String>
where
    MSG: WithLogMessage,
{
    let config_folder = dirs::get_configs_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| "Failed to convert config path to string")?;

    let ws_foler = dirs::get_wireless_stack_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| "Failed to convert config path to string")?;

    let tmp_folder = dirs::get_tmp_dir()?
        .into_os_string()
        .into_string()
        .map_err(|_| "Failed to convert tmp_dir to string.")?;

    let cmd = cmd.args(&[
        "-s",
        "scripts",
        "-s",
        &config_folder,
        "-s",
        &tmp_folder,
        "-s",
        &ws_foler,
    ]);

    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stderr = child.stderr.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    let mutex_messages: Arc<Mutex<VecDeque<LogType>>> = Arc::new(Mutex::new(VecDeque::new()));

    let thread_message = mutex_messages.clone();
    let thread_stdout = thread::spawn(move || {
        let lines = BufReader::new(stderr).lines();

        for line in lines {
            match line {
                Ok(line) => {
                    println!("[OPEN OCD] {line}");
                    thread_message
                        .lock()
                        .unwrap()
                        .push_back(LogType::Info(format!("    {line}")));
                }
                Err(_) => (),
            }
        }
    });

    let thread_message = mutex_messages.clone();
    let thread_stderr = thread::spawn(move || {
        let lines = BufReader::new(stdout).lines();

        for line in lines {
            match line {
                Ok(line) => {
                    eprintln!("[OPEN OCD] {line}");
                    thread_message
                        .lock()
                        .unwrap()
                        .push_back(LogType::Error(format!("    {line}")));
                }
                Err(_) => (),
            }
        }
    });

    let logs = LogEntries::default();
    let output: ExitStatus;
    let mut tmp_deque: VecDeque<LogType> = VecDeque::new();

    loop {
        if let Ok(mut deque) = mutex_messages.lock() {
            tmp_deque = deque.clone();
            deque.clear();
        }

        while let Some(msg) = tmp_deque.pop_front() {
            if let Some(ref mut s) = sender {
                let _ = s.send(MSG::log(msg)).await;
            } else {
                logs.push(msg);
            }
        }

        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            output = status;
            let _ = thread_stdout.join();
            let _ = thread_stderr.join();
            break;
        }
    }

    if let Some(ref mut s) = sender {
        let _ = s
            .send(MSG::log(LogType::Warning(format!(
                "Exit code: {}",
                output.code().unwrap_or(i32::MIN)
            ))))
            .await;
    }

    Ok(ProcessResult {
        code: output.code(),
        log: logs,
    })
}
