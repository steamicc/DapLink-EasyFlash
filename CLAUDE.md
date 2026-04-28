# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Internal tool to flash DAPLink onto an STM32F103xB interface chip on boards like STeaMi, STM32 Disco L475 IoTNode, and STM32 Nucleo WB55, replacing the need for ST-LINK utility. The flow chains OpenOCD invocations and USB mass-storage drops:

1. RDP unlock (OpenOCD) → 2. mass erase (OpenOCD) → 3. flash bootloader (OpenOCD) → 4. wait for `MAINTENANCE` USB drive, copy firmware to it → 5. (optional) wait for the user-named target drive (e.g. `DAPLINK`, `STEAMI`, `DIS_L4IOT`), copy user program to it.

## Build & Run

```bash
cargo run                       # dev run
cargo build --release --locked  # release build (matches CI)
```

System dependencies on Linux (from CI): `libatk1.0-dev pkg-config libgtk-3-dev` (GTK is required by the `rfd` file dialog).

The `openocd` binary must be on `PATH` at runtime. Releases bundle [xpack-openocd v0.12.0-4](https://github.com/xpack-dev-tools/openocd-xpack); when running from sources you must install it yourself.

Windows release builds are produced via `cross build --target x86_64-pc-windows-gnu`. macOS is unsupported.

There is no test suite — `cargo test` runs nothing meaningful.

## Architecture

GUI app built on **iced 0.13** using its Elm-style `application(title, update, view)` pattern. Entry point is `src/main.rs`; the main state lives in `EasyDapLink` (`src/main_widget.rs`).

### Async pipeline as a Message chain

The flashing workflow is **not** a single async function. Each stage is a separate `Message` variant in `src/messages.rs`, and `EasyDapLink::update` is a state machine that, on success of stage N, returns `Task::perform(stage_N+1, Message::DoneStageN+1)`. The chain is:

`StartProcess → DoneUnlockProcess → DoneEraseProcess → DoneFlashProcess → DoneWaitMaintenanceDisk → DoneCopyFirmware → DoneWaitingDeviceDisk → DoneCopyUserfile → DoneProcess`

When adding or modifying a stage, edit both `Message` and the corresponding arm in `update`; do not try to fold stages into a single async fn — `is_readonly`/log updates depend on returning to the iced runtime between steps.

### OpenOCD scripts: embedded then materialized

The three `.cfg` files in `configs/` are embedded into the binary via `include_str!` in `src/open_ocd_task.rs`. On startup, `create_script_file()` writes them out to the platform data directory (`ProjectDirs("cc", "steami", "daplink-easyflash")/scripts/`). All `openocd` invocations reference those on-disk copies via `-f`, plus `-s scripts` for relative `find` lookups in OpenOCD scripts. The bootloader binary is also copied into a sibling `tmp/` dir before flashing because the OpenOCD `program` command is run with `-s <tmp>`.

If you change a `.cfg` you must reinstall (or delete the cached script directory) for the new contents to take effect, since `create_script_file` overwrites unconditionally on each launch — but users who previously ran the app will still pick up the new version next launch.

### Disk detection

`src/disk_tool.rs` uses `sysinfo::Disks` and identifies drives by **mount point file name** on Unix (e.g. `/media/user/MAINTENANCE` → `MAINTENANCE`) and by **disk name** on Windows. The `MAINTENANCE` constant in `main_widget.rs` is the DAPLink bootloader-mode volume and is not user-configurable; the second drop target (`target_name`) is.

### Settings persistence

`EasyDapLink` derives `Serialize`/`Deserialize`. On window close (`Event::Window(CloseRequested)`), it serializes itself to `<settings_dir>/fields.json`; on startup `Default::default()` calls `load_fields()` and merges saved paths/target name back. Fields marked `#[serde(skip)]` (`theme`, `is_readonly`, `log_widget`) are intentionally not persisted. Because close is intercepted, the app sets `exit_on_close_request(false)` and explicitly calls `iced::window::close` after saving.

## Key files

- [src/main_widget.rs](src/main_widget.rs) — state, `update` state machine, `view`, settings load/save
- [src/open_ocd_task.rs](src/open_ocd_task.rs) — OpenOCD invocation, script materialization, stdout/stderr line capture
- [src/disk_tool.rs](src/disk_tool.rs) — mount-point polling and file copy
- [src/dirs.rs](src/dirs.rs) — `scripts/`, `tmp/`, `settings/` under platform data dir
- [configs/](configs/) — OpenOCD scripts embedded at compile time
