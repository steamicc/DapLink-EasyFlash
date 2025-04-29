use std::{
    cmp::Ordering,
    fs,
    future::Future,
    io::{self, Write},
    path::{Path, PathBuf},
    thread::{self, sleep},
    time::Duration,
};

use iced::{
    alignment::Horizontal,
    futures::{
        channel::mpsc::{self, Sender},
        SinkExt,
    },
    stream::channel,
    widget::{button, center, column, container, opaque, pick_list, row, stack, text},
    Element, Length, Task, Theme,
};
use iced_aw::{grid, grid_row};
use serde::{Deserialize, Serialize};
use serialport::{SerialPort, SerialPortType};

use crate::{
    dirs,
    log_entries::{LogEntries, LogType},
    open_ocd_task,
    operator_tool::{
        operator_error_string, upgrade_status_string, OperatorResult, OperatorVersionResult,
    },
    stackfile_config::{fus_config, wireless_stack_config, FusFile, WirelessStackFile},
};

use super::{
    log_widget::LogWidget,
    messages::{Message, TabWsMessage},
};

#[derive(Debug, Default, Clone)]
pub enum FwStep {
    #[default]
    Ready,
    StartProcess,
    StepFlashOperator,
    StepUpgradeFUS,
    StepFlashFUS(String),
    StepDeleteFW,
    StepFlashFW,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SerialPortInfo {
    port: String,
    product: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabWirelessStack {
    fw_selected: WirelessStackFile,
    #[serde(skip)]
    serial_available_port: Vec<SerialPortInfo>,
    #[serde(skip)]
    serial_selected: Option<SerialPortInfo>,
    #[serde(skip)]
    log: LogWidget,
    #[serde(skip)]
    is_readonly: bool,
}

const ALL_STACK: [WirelessStackFile; 20] = [
    WirelessStackFile::BleHciAdvScan,
    WirelessStackFile::BleHciExt,
    WirelessStackFile::BleHci,
    WirelessStackFile::BleMac,
    WirelessStackFile::BleStackFullExt,
    WirelessStackFile::BleStackFull,
    WirelessStackFile::BleStackLight,
    WirelessStackFile::BleThreadDyn,
    WirelessStackFile::BleThreadSta,
    WirelessStackFile::BleZigbeeFfdDyn,
    WirelessStackFile::BleZigbeeFfdSta,
    WirelessStackFile::BleZigbeeRfdDyn,
    WirelessStackFile::BleZigbeeRfdSta,
    WirelessStackFile::Mac802154,
    WirelessStackFile::Phy802154,
    WirelessStackFile::ThreadFtd,
    WirelessStackFile::ThreadMtd,
    WirelessStackFile::ThreadRcp,
    WirelessStackFile::ZigbeeFfd,
    WirelessStackFile::ZigbeeRfd,
];

impl TabWirelessStack {
    pub fn view(&self) -> Element<Message> {
        let grid_fields = grid!(
            grid_row!(
                "Wireless Stack",
                pick_list(&ALL_STACK[..], Some(&self.fw_selected), |x| {
                    Message::WirelessStack(TabWsMessage::StackSelected(x))
                })
                .width(Length::Fill)
            ),
            grid_row!(
                "Serial port",
                row![
                    pick_list(
                        &self.serial_available_port[..],
                        self.serial_selected.as_ref(),
                        |x| Message::WirelessStack(TabWsMessage::SerialSelected(x))
                    )
                    .width(Length::Fill),
                    button(text("Refresh"))
                        .on_press(Message::WirelessStack(TabWsMessage::SerialRefresh))
                ]
                .spacing(8)
            ),
        )
        .width(Length::Fill)
        .column_spacing(8)
        .row_spacing(8)
        .column_widths(&[Length::Shrink, Length::Fill]);

        let start_button = button(
            text("Start 🚀")
                .shaping(text::Shaping::Advanced)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .on_press(Message::WirelessStack(TabWsMessage::StepChange(
            FwStep::StartProcess,
        )))
        .width(Length::Fill);

        let log = container(self.log.view())
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(8);

        let main_col = column![grid_fields, start_button].padding(8).spacing(16);

        let layout = if self.is_readonly {
            column![
                stack![
                    main_col,
                    opaque(center(text("")).style(|theme: &Theme| {
                        let mut bg = theme.palette().background;
                        bg.a = 0.8;
                        container::Style {
                            background: Some(bg.into()),
                            ..container::Style::default()
                        }
                    }))
                ],
                log
            ]
        } else {
            column![main_col, log]
        };

        layout.into()
    }

    pub fn update(&mut self, message: TabWsMessage) -> Task<Message> {
        match message {
            TabWsMessage::StackSelected(file) => self.fw_selected = file,
            TabWsMessage::SerialSelected(serial) => self.serial_selected = Some(serial),
            TabWsMessage::SerialRefresh => {
                self.refresh_serial_ports();
            }
            TabWsMessage::StepChange(next_step) => {
                return match next_step {
                    FwStep::Ready => {
                        self.is_readonly = false;
                        Task::none()
                    }
                    FwStep::StartProcess => self.step_start_process(),
                    FwStep::StepFlashOperator => self.step_flash_operator(),
                    FwStep::StepUpgradeFUS => self.step_upgrade_fus(),
                    FwStep::StepFlashFUS(file) => self.step_flash_fus(file),
                    FwStep::StepDeleteFW => self.step_delete_fw(),
                    FwStep::StepFlashFW => self.step_flash_fw(),
                };
            }
            TabWsMessage::LogMessage(log) => self.log.push(log),
            TabWsMessage::LogMessages(entries) => self.log.from_log_entries(&entries),
        }

        Task::none()
    }

    fn step_start_process(&mut self) -> Task<Message> {
        if self.serial_selected.is_none() {
            self.log
                .push(LogType::Error("Please select a serial port".into()));

            return Task::none();
        }

        self.is_readonly = true;
        self.log
            .push(LogType::Info("Start flashing...".to_string()));

        let serial = self.serial_selected.as_ref().unwrap().clone();
        Self::message_runner(|mut o| async move {
            match Self::test_serial_port(&serial.port) {
                Ok(_) => Self::send_step(&mut o, FwStep::StepFlashOperator).await,
                Err(e) => Self::error_handle(&mut o, e).await,
            };
        })
    }

    fn step_flash_operator(&mut self) -> Task<Message> {
        self.log.push(LogType::Info("Flash operator".to_string()));

        Self::message_runner(|mut o| async move {
            match open_ocd_task::flash_wb55("wb55_operator.hex", 0).await {
                Ok(result) => {
                    if result.code.is_some() && result.code.unwrap() != 0 {
                        Self::send_log(&mut o, LogType::Error("Flash failed".into())).await;
                        Self::send_step(&mut o, FwStep::Ready).await;
                    } else {
                        Self::send_logs(&mut o, result.log).await;
                        Self::send_step(&mut o, FwStep::StepUpgradeFUS).await;
                    }
                }
                Err(e) => Self::error_handle(&mut o, e).await,
            }
        })
    }

    fn step_upgrade_fus(&mut self) -> Task<Message> {
        self.log.push(LogType::Info("FUS update".to_string()));

        let serial = self.serial_selected.as_ref().unwrap().clone();
        Self::message_runner(|mut o| async move {
            thread::sleep(Duration::from_secs(1));

            let mut port = match Self::open_port(&serial.port) {
                Ok(p) => p,
                Err(e) => {
                    Self::error_handle(&mut o, format!("Failed to open serial port. Error: {e}"))
                        .await;
                    return;
                }
            };

            if let Err(e) = Self::send_double_status(&mut port, &mut o).await {
                Self::error_handle(&mut o, e).await;
                return;
            }

            let line =
                match Self::send_and_read_serial(&mut port, "VERSION\n".as_bytes(), None, None) {
                    Ok(s) => s,
                    Err(e) => {
                        Self::error_handle(&mut o, e).await;
                        return;
                    }
                };
            let version = match Self::parse_result::<OperatorVersionResult>(&line) {
                Ok(obj) => obj,
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            let major = (version.fus_version & 0xFF000000) >> 24;
            let minor = (version.fus_version & 0x00FF0000) >> 16;

            let mut fus: Option<&str> = None;

            if major == 0x00 {
                fus = Some(fus_config(FusFile::FusFor0_5_3));
            } else if major == 0x01 && minor < 0x02 {
                fus = Some(fus_config(FusFile::Fus1_2_0));
            } else if major == 0x01 && minor == 0x02 {
                Self::send_log(&mut o, LogType::Info("FUS is up to date".to_string())).await;
            } else if major == 0x02 {
                Self::send_log(
                    &mut o,
                    LogType::Warning(
                        "FUS is ahead ! Let's give it a try. But it could fail...".to_string(),
                    ),
                )
                .await;
            } else {
                Self::send_log(
                    &mut o,
                    LogType::Error("Unknown FUS version. Abort.".to_string()),
                )
                .await;
                Self::send_step(&mut o, FwStep::Ready).await;
                return;
            }

            if let None = fus {}

            match fus {
                Some(file) => Self::send_step(&mut o, FwStep::StepFlashFUS(file.to_string())).await,
                None => Self::send_step(&mut o, FwStep::StepDeleteFW).await,
            };
        })
    }

    fn step_flash_fus(&mut self, file: String) -> Task<Message> {
        self.log.push(LogType::Info("Flash FUS".to_string()));

        let serial = self.serial_selected.as_ref().unwrap().clone();
        Self::message_runner(|mut o| async move {
            let path_op = match Self::path_ws_file("wb55_operator_no_end.hex") {
                Ok(path) => path,
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            let path_fus = match Self::path_ws_file(&file) {
                Ok(path) => path,
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            let path_result = match dirs::get_tmp_dir() {
                Ok(mut path) => {
                    path.push("merge.hex");
                    path
                }
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            if let Err(e) = Self::merge_ws_hex(&path_op, &path_fus, &path_result) {
                Self::error_handle(&mut o, e).await;
                return;
            }

            match open_ocd_task::flash_wb55("merge.hex", 0).await {
                Ok(result) => {
                    if result.code.is_some() && result.code.unwrap() != 0 {
                        Self::send_log(&mut o, LogType::Error("Flash failed".into())).await;
                        Self::send_step(&mut o, FwStep::Ready).await;
                        return;
                    } else {
                        Self::send_logs(&mut o, result.log).await;
                    }
                }
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            }

            Self::send_log(&mut o, LogType::Info("Send UPGRADE command".into())).await;
            match Self::fus_upgrade_cmd(&serial).await {
                Ok(_) => Self::send_step(&mut o, FwStep::StepUpgradeFUS).await,
                Err(e) => Self::error_handle(&mut o, e).await,
            }
        })
    }

    fn step_delete_fw(&mut self) -> Task<Message> {
        self.log
            .push(LogType::Info("Delete current wireless stack".to_string()));

        let serial = self.serial_selected.as_ref().unwrap().clone();
        Self::message_runner(|mut o| async move {
            let mut port = match Self::open_port(&serial.port) {
                Ok(port) => port,
                Err(e) => {
                    Self::send_log(
                        &mut o,
                        LogType::Error(format!("Failed to open serial port. Error: {e}")),
                    )
                    .await;
                    Self::send_step(&mut o, FwStep::Ready).await;
                    return;
                }
            };

            let mut success = false;
            for attempt in 0..3 {
                thread::sleep(Duration::from_secs(1));
                match Self::send_and_read_serial(&mut port, "DELETE\n".as_bytes(), None, None) {
                    Ok(_) => {
                        success = true;
                        break;
                    }
                    Err(e) => {
                        Self::send_log(
                            &mut o,
                            LogType::Warning(format!(
                                "Delete attempt #{} failed. Error: {e}",
                                attempt + 1
                            )),
                        )
                        .await
                    }
                }
            }

            if !success {
                Self::send_log(
                    &mut o,
                    LogType::Error("Unable to send delete command.".to_string()),
                )
                .await;
                Self::send_step(&mut o, FwStep::Ready).await;
                return;
            }

            match Self::send_double_status(&mut port, &mut o).await {
                Ok(_) => Self::send_step(&mut o, FwStep::StepFlashFW).await,
                Err(e) => Self::error_handle(&mut o, e).await,
            };
        })
    }

    fn step_flash_fw(&mut self) -> Task<Message> {
        self.log
            .push(LogType::Info("Flash wireless stack".to_string()));

        let serial = self.serial_selected.as_ref().unwrap().clone();
        let fw = wireless_stack_config(self.fw_selected);
        Self::message_runner(move |mut o| async move {
            let path_op = match Self::path_ws_file("wb55_operator_no_end.hex") {
                Ok(path) => path,
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            let path_fw = match Self::path_ws_file(fw) {
                Ok(path) => path,
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            let path_result = match dirs::get_tmp_dir() {
                Ok(mut path) => {
                    path.push("merge.hex");
                    path
                }
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            if let Err(e) = Self::merge_ws_hex(&path_op, &path_fw, &path_result) {
                Self::error_handle(&mut o, e).await;
                return;
            }

            match open_ocd_task::flash_wb55("merge.hex", 0).await {
                Ok(result) => {
                    if result.code.is_some() && result.code.unwrap() != 0 {
                        Self::send_log(&mut o, LogType::Error("Flash failed".into())).await;
                        Self::send_step(&mut o, FwStep::Ready).await;
                        return;
                    } else {
                        Self::send_logs(&mut o, result.log).await;
                    }
                }
                Err(e) => {
                    Self::error_handle(&mut o, e).await;
                    return;
                }
            };

            Self::send_log(&mut o, LogType::Info("Send UPGRADE command".into())).await;
            match Self::fus_upgrade_cmd(&serial).await {
                Ok(_) => {
                    Self::send_log(
                        &mut o,
                        LogType::Info("Wireless stack is now flashed !".into()),
                    )
                    .await;
                    Self::send_step(&mut o, FwStep::Ready).await;
                }
                Err(e) => Self::error_handle(&mut o, e).await,
            }
        })
    }

    fn message_runner<F>(f: impl FnOnce(mpsc::Sender<TabWsMessage>) -> F + 'static) -> Task<Message>
    where
        F: Future<Output = ()> + std::marker::Send + 'static,
    {
        Task::run(channel(1, f), |x| Message::WirelessStack(x))
    }

    fn merge_ws_hex(first: &Path, second: &Path, result: &Path) -> Result<(), String> {
        let mut result_file = fs::OpenOptions::new()
            .append(false)
            .write(true)
            .truncate(true)
            .create(true)
            .open(result)
            .map_err(|e| {
                format!(
                    "Failed to open 'result' path '{}'. Error: {e}",
                    result.to_str().unwrap_or("undefined")
                )
            })?;

        let mut first_file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(first)
            .map_err(|e| {
                format!(
                    "Failed to open 'first' path '{}'. Error: {e}",
                    first.to_str().unwrap_or("undefined")
                )
            })?;

        let mut second_file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(second)
            .map_err(|e| {
                format!(
                    "Failed to open 'second' path '{}'. Error: {e}",
                    second.to_str().unwrap_or("undefined")
                )
            })?;

        io::copy(&mut first_file, &mut result_file)
            .map_err(|e| format!("Failed to copy first file to result file. Error: {e}"))?;
        io::copy(&mut second_file, &mut result_file)
            .map_err(|e| format!("Failed to copy second file to result file. Error: {e}"))?;

        Ok(())
    }

    async fn send_logs(o: &mut Sender<TabWsMessage>, logs: LogEntries) {
        let _ = o.send(TabWsMessage::LogMessages(logs)).await;
    }

    async fn send_log(o: &mut Sender<TabWsMessage>, log: LogType) {
        let _ = o.send(TabWsMessage::LogMessage(log)).await;
    }

    async fn send_step(o: &mut Sender<TabWsMessage>, step: FwStep) {
        let _ = o.send(TabWsMessage::StepChange(step)).await;
    }

    async fn error_handle(o: &mut Sender<TabWsMessage>, error: String) {
        Self::send_log(o, LogType::Error(error)).await;
        Self::send_step(o, FwStep::Ready).await;
    }

    fn path_ws_file(filename: &str) -> Result<PathBuf, String> {
        let mut path = dirs::get_wireless_stack_dir()?;
        path.push(filename);
        Ok(path)
    }

    pub fn refresh_serial_ports(&mut self) {
        let ports = serialport::available_ports();

        self.serial_available_port.clear();
        self.serial_selected = None;

        if let Ok(ports) = ports {
            for p in ports {
                let mut port_helper = SerialPortInfo {
                    port: p.port_name,
                    product: None,
                };

                if let SerialPortType::UsbPort(type_port) = p.port_type {
                    if let Some(product) = type_port.product {
                        port_helper.product = Some(product);
                    }
                }

                self.serial_available_port.push(port_helper);
            }

            self.serial_available_port.sort_by(|a, b| {
                if a.product.is_none() && b.product.is_some() {
                    return Ordering::Greater;
                } else if a.product.is_some() && b.product.is_none() {
                    return Ordering::Less;
                } else {
                    a.port.cmp(&b.port)
                }
            });

            self.serial_selected = Some(self.serial_available_port[0].clone())
        }
    }

    fn open_port(port: &str) -> serialport::Result<Box<dyn SerialPort>> {
        serialport::new(port, 115_200)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(10))
            .open()
    }

    fn test_serial_port(port: &str) -> Result<(), String> {
        match Self::open_port(port) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(format!(
                    "Failed to open serial port \"{}\". Error: {}",
                    port, e
                ));
            }
        }
    }

    /// Sends data over serial, wait a certain amount of time (1 sec by default, if `wait_time` is `None`) and read data from serial.
    /// The number of bytes read is retruned.
    fn send_and_read_serial(
        port: &mut Box<dyn SerialPort>,
        send_buf: &[u8],
        wait_time: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Result<String, String> {
        if let Err(e) = port.write_all(send_buf) {
            return Err(format!("Serial write failed: {}", e.to_string()));
        }

        port.flush().map_err(|_| "Failed to flush serial.")?;

        let old_timeout = port.timeout();

        if let Some(ref t) = timeout {
            port.set_timeout(t.clone()).map_err(|x| x.to_string())?;
        }

        sleep(wait_time.unwrap_or(Duration::from_secs(1)));

        let read = Self::read_line(port, None);

        if timeout.is_some() {
            port.set_timeout(old_timeout).map_err(|x| x.to_string())?;
        }

        match read {
            Ok(string) => Ok(string),
            Err(e) => Err(format!("Failed to read data. Error: {}", e)),
        }
    }

    fn read_line(
        port: &mut Box<dyn SerialPort>,
        timeout: Option<Duration>,
    ) -> Result<String, String> {
        let old_timeout = port.timeout();

        if let Some(ref t) = timeout {
            port.set_timeout(t.clone()).map_err(|x| x.to_string())?;
        }

        let mut buf = [0];
        let mut result = String::new();

        while port
            .read(&mut buf)
            .map_err(|e| format!("Failed to read data. Error: {e}"))?
            > 0
        {
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }

            result.push(c);
        }

        if timeout.is_some() {
            port.set_timeout(old_timeout).map_err(|x| x.to_string())?;
        }

        Ok(result)
    }

    fn parse_result<'a, T: serde::Deserialize<'a>>(result: &'a String) -> Result<T, String> {
        match serde_json::from_str(result) {
            Ok(obj) => Ok(obj),
            Err(e) => Err(format!("Failed to parse json. Error: {e}")),
        }
    }

    async fn send_double_status(
        port: &mut Box<dyn SerialPort>,
        o: &mut Sender<TabWsMessage>,
    ) -> Result<(), String> {
        let mut success = false;
        for nb in 0..2 {
            for attempt in 0..3 {
                match Self::send_and_read_serial(port, "STATUS\n".as_bytes(), None, None) {
                    Ok(_) => {
                        success = true;
                        break;
                    }
                    Err(e) => {
                        Self::send_log(
                            o,
                            LogType::Warning(format!(
                                "STATUS #{}, attempt #{} failed (Error: {e}.",
                                nb + 1,
                                attempt + 1
                            )),
                        )
                        .await;
                    }
                }
            }
        }

        if success {
            Ok(())
        } else {
            Err("Unable to unlock FUS.".into())
        }
    }

    async fn fus_upgrade_cmd(port_info: &SerialPortInfo) -> Result<(), String> {
        let mut port = match Self::open_port(&port_info.port) {
            Ok(port) => port,
            Err(e) => {
                return Err(format!("Failed to open serial port. Error: {e}"));
            }
        };

        thread::sleep(Duration::from_secs(5));

        port.write("UPGRADE\n".as_bytes())
            .map_err(|e| format!("Failed to write serial. Error: {e}"))?;
        port.flush()
            .map_err(|e| format!("Failed to flush serial. Error: {e}"))?;

        let mut errors = String::new();
        loop {
            let line = Self::read_line(&mut port, Some(Duration::from_secs(10)))?;
            let result: OperatorResult = Self::parse_result(&line)?;

            println!(
                "[upgrade] status: {} ({})  |  error: {} ({})",
                result.status,
                upgrade_status_string(result.status),
                result.error.unwrap_or(0),
                operator_error_string(result.error.unwrap_or(0))
            );

            if let Some(ref err) = result.error {
                if *err != 0 {
                    errors += &format!("{}\r\n", operator_error_string(*err));
                }
            }

            if result.status == 0 {
                break;
            }
        }

        if errors.len() == 0 {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Default for TabWirelessStack {
    fn default() -> Self {
        let mut s = Self {
            fw_selected: Default::default(),
            serial_available_port: Default::default(),
            serial_selected: Default::default(),
            log: Default::default(),
            is_readonly: false,
        };

        s.refresh_serial_ports();

        s
    }
}

impl std::fmt::Display for SerialPortInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(product) = self.product.as_ref() {
            f.write_str(&format!("{} - {}", &self.port, product))
        } else {
            f.write_str(&self.port)
        }
    }
}
