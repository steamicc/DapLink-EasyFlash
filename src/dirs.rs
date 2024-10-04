use std::{fs, path::PathBuf};

use directories::ProjectDirs;

fn get_base_dir() -> Result<PathBuf, String> {
    match ProjectDirs::from("eu", "letssteam", "daplink-easyflash") {
        Some(b) => Ok(b.data_dir().to_path_buf()),
        None => {
            Err("No valid home directory path could be retrieved from the operating system".into())
        }
    }
}

pub fn get_script_dir() -> Result<PathBuf, String> {
    let base = get_base_dir()?;

    let script = base.join("scripts");

    if !script.exists() && fs::create_dir_all(&script).is_err() {
        match fs::create_dir_all(&script) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Failed to create script directory {e}"));
            }
        }
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
                return Err(format!("Failed to create script directory {e}"));
            }
        }
    }

    Ok(tmp)
}
