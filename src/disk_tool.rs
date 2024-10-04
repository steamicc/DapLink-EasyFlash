use std::{
    fs,
    path::PathBuf,
    thread,
    time::{Duration, SystemTime},
};

use sysinfo::Disks;

#[derive(Debug, Clone)]
pub struct DiskResult {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_list_disks() -> Vec<DiskResult> {
    let mut result = Vec::new();

    for disk in Disks::new_with_refreshed_list().list() {
        #[cfg(target_os = "windows")]
        match disk.name().to_str() {
            Some(s) => result.push(DiskResult {
                name: s.into(),
                path: disk.mount_point().into(),
            }),
            None => (),
        };

        #[cfg(not(target_os = "windows"))]
        match disk.mount_point().file_name().and_then(|f| f.to_str()) {
            Some(s) => result.push(DiskResult {
                name: s.into(),
                path: disk.mount_point().into(),
            }),
            None => (),
        };
    }

    result
}

pub async fn copy_file_to_disk(disk_name: String, file: PathBuf) -> Result<(), String> {
    let disk = match get_list_disks().iter().find(|x| x.name == *disk_name) {
        Some(d) => d.clone(),
        None => return Err(format!("Unable to find '{disk_name}' disk.")),
    };

    match thread::spawn(move || {
        fs::copy(
            &file,
            disk.path.join(file.file_name().unwrap().to_str().unwrap()),
        )
    })
    .join()
    {
        Ok(r) => match r {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Copy error : {e}")),
        },
        Err(e) => Err(format!("Copy thread error: {:#?}", e)),
    }
}

pub async fn wait_for_drive(disk_name: String, timeout: Duration) -> bool {
    let start = SystemTime::now();

    while start.elapsed().unwrap() <= timeout {
        let disk_list = get_list_disks();

        if disk_list.iter().any(|x| x.name == *disk_name) {
            return true;
        }

        thread::sleep(Duration::from_millis(500));
    }

    false
}
