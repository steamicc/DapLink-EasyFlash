use std::path::PathBuf;

use rfd::AsyncFileDialog;

pub async fn select_file(current: PathBuf, title: &str, allow_hex: bool) -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new()
        .set_title(title)
        .add_filter("Binary file (*.bin)", &["bin", "BIN"]);

    if allow_hex {
        dialog = dialog.add_filter("Hex file (*.hex)", &["hex", "HEX"]);
    }

    dialog = dialog.add_filter("All file", &["*"]);

    if current.exists() {
        if current.is_dir() {
            dialog = dialog.set_directory(current);
        } else {
            match current.parent() {
                Some(p) => dialog = dialog.set_directory(p),
                None => (),
            };
        }
    }

    dialog.pick_file().await.map(|h| h.path().to_path_buf())
}
