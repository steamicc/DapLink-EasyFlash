use std::{
    fs, i32,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    dirs,
    log_entries::{LogEntries, LogType},
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

    Ok(run_command(&mut command).await?)
}

pub async fn erase_target() -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_configs_dir()?;
    let path_script = script_folder.join(ERASE_SCRIPT_FILENAME);

    let mut command = Command::new("openocd");
    command.args(&["-f", &format!("{}", path_script.to_str().unwrap())]);

    Ok(run_command(&mut command).await?)
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

    Ok(run_command(&mut command).await?)
}

pub fn is_installed() -> Result<bool, String> {
    let child = Command::new("openocd")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    Ok(child.status.success())
}

pub async fn flash_wb55(file: &str, _offset: u32) -> Result<ProcessResult, String> {
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

    run_command(&mut command).await
}

//TODO : Really need a ref. here ?
async fn run_command(cmd: &mut Command) -> Result<ProcessResult, String> {
    let logs = LogEntries::default();

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

    let mutex_log = Arc::new(Mutex::new(logs));

    let thread_mutex = mutex_log.clone();
    let thread_stdout = thread::spawn(move || {
        let lines = BufReader::new(stderr).lines();

        for line in lines {
            match line {
                Ok(line) => {
                    println!("[OPEN OCD] {line}");
                    thread_mutex.lock().unwrap().push(LogType::Info(line));
                }
                Err(_) => (),
            }
        }
    });

    let thread_mutex = mutex_log.clone();
    let thread_stderr = thread::spawn(move || {
        let lines = BufReader::new(stdout).lines();

        for line in lines {
            match line {
                Ok(line) => {
                    eprintln!("[OPEN OCD] {line}");
                    thread_mutex.lock().unwrap().push(LogType::Info(line))
                }
                Err(_) => (),
            }
        }
    });

    let output = child.wait().map_err(|e| e.to_string())?;
    let _ = thread_stdout.join();
    let _ = thread_stderr.join();

    let logs = match Arc::into_inner(mutex_log) {
        Some(m) => match m.into_inner() {
            Ok(l) => l,
            Err(e) => return Err(format!("Unable to get inner mutex ({e})")),
        },
        None => return Err("Unable te get inner Arc".into()),
    };

    logs.push(LogType::Warning(format!(
        "Exit code: {}",
        output.code().unwrap_or(i32::MIN)
    )));

    Ok(ProcessResult {
        code: output.code(),
        log: logs,
    })
}
