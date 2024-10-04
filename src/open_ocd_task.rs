use std::{
    fs,
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

const UNLOCK_SCRIPT_FILENAME: &str = "openocd-unlock.cfg";
const ERASE_SCRIPT_FILENAME: &str = "openocd-erase.cfg";
const FLASH_SCRIPT_FILENAME: &str = "openocd-flash.cfg";

const UNLOCK_SCRIPT: &str = include_str!("../configs/openocd-unlock.cfg");
const ERASE_SCRIPT: &str = include_str!("../configs/openocd-mass-erase.cfg");
const FLASH_SCRIPT: &str = include_str!("../configs/openocd-flash.cfg");

pub async fn unlock_target() -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_script_dir()?;
    let path_script = script_folder.join(UNLOCK_SCRIPT_FILENAME);

    let mut command = Command::new("openocd");
    command.args(&["-f", &format!("{}", path_script.to_str().unwrap())]);

    println!("{:?}", command.get_args());

    Ok(run_command(&mut command).await?)
}

pub async fn erase_target() -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_script_dir()?;
    let path_script = script_folder.join(ERASE_SCRIPT_FILENAME);

    let mut command = Command::new("openocd");
    command.args(&["-f", &format!("{}", path_script.to_str().unwrap())]);

    Ok(run_command(&mut command).await?)
}

pub async fn flash_target(bin_path: PathBuf) -> Result<ProcessResult, String> {
    let script_folder: &Path = &dirs::get_script_dir()?;
    let path_script = script_folder.join(FLASH_SCRIPT_FILENAME);

    if !bin_path.is_file() {
        return Err("The firmware path is not a file.".into());
    }

    match fs::copy(bin_path, dirs::get_tmp_dir()?.join("bootloader")) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to copy bootloader file ({e}")),
    };

    let mut command = Command::new("openocd");
    command.args(&["-f", &format!("{}", path_script.to_str().unwrap())]);

    Ok(run_command(&mut command).await?)
}

pub fn is_installed() -> Result<bool, String> {
    let child = Command::new("openocd")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    Ok(child.status.success())
}

pub fn create_script_file() -> Result<(), String> {
    let script_folder: &Path = &dirs::get_script_dir()?;

    let path_unlock = script_folder.join(UNLOCK_SCRIPT_FILENAME);
    let path_erase = script_folder.join(ERASE_SCRIPT_FILENAME);
    let path_flash = script_folder.join(FLASH_SCRIPT_FILENAME);

    fs::write(path_unlock, UNLOCK_SCRIPT)
        .and_then(move |_| fs::write(path_erase, ERASE_SCRIPT))
        .and_then(move |_| fs::write(path_flash, FLASH_SCRIPT))
        .map_err(|e| format!("{e}"))?;

    Ok(())
}

async fn run_command(cmd: &mut Command) -> Result<ProcessResult, String> {
    let logs = LogEntries::default();

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

    Ok(ProcessResult {
        code: output.code(),
        log: logs,
    })
}
