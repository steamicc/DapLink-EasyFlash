use std::{fs, path::PathBuf};

use directories::ProjectDirs;

static mut BASE_PATH: Option<PathBuf> = None;

fn get_base_dir() -> Result<PathBuf, String> {
    match ProjectDirs::from("cc", "steami", "daplink-easyflash") {
        Some(b) => Ok(b.data_dir().to_path_buf()),
        None => {
            Err("No valid home directory path could be retrieved from the operating system".into())
        }
    }
}

pub fn set_exe_dir(pathbuf: PathBuf) {
    unsafe {
        BASE_PATH = Some(pathbuf);
    }
}

pub fn get_exe_dir() -> Result<PathBuf, String> {
    unsafe {
        #[allow(static_mut_refs)]
        match BASE_PATH.as_ref() {
            Some(path) => Ok(path.clone()),
            None => Err("No base path available. This error should not be happen !".into()),
        }
    }
}

pub fn get_configs_dir() -> Result<PathBuf, String> {
    let base = get_exe_dir()?;

    let script = base.join("configs");

    if !script.exists() {
        return Err(format!(
            "The configs folder does not exist (exe_dir: {})",
            base.to_str().unwrap_or("not defined")
        ));
    }

    Ok(script)
}

pub fn get_wireless_stack_dir() -> Result<PathBuf, String> {
    let base = get_exe_dir()?;

    let script = base.join("wireless_stack");

    if !script.exists() {
        return Err(format!(
            "The wireless_stack folder does not exist (exe_dir: {})",
            base.to_str().unwrap_or("not defined")
        ));
    }

    Ok(script)
}

pub fn get_tmp_dir() -> Result<PathBuf, String> {
    let base = get_base_dir()?;

    let tmp = base.join("tmp");

    if !tmp.exists() && fs::create_dir_all(&tmp).is_err() {
        match fs::create_dir_all(&tmp) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Failed to create tmp directory {e}"));
            }
        }
    }

    Ok(tmp)
}

pub fn get_settings_dir() -> Result<PathBuf, String> {
    let base = get_base_dir()?;

    let tmp = base.join("settings");

    if !tmp.exists() && fs::create_dir_all(&tmp).is_err() {
        match fs::create_dir_all(&tmp) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Failed to create settings directory {e}"));
            }
        }
    }

    Ok(tmp)
}
