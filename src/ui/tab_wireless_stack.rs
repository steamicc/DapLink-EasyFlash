use iced::{
    alignment::Horizontal,
    widget::{button, center, column, container, opaque, pick_list, stack, text},
    Element, Length, Task, Theme,
};
use iced_aw::{grid, grid_row};

use crate::log_entries::LogType;

use super::{
    log_widget::LogWidget,
    messages::{Message, TabWirelessStackMessage},
};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WirelessStackFile {
    BleHciAdvScan,
    #[default]
    BleHciExt,
    BleHci,
    BleMac,
    BleStackFullExt,
    BleStackFull,
    BleStackLight,
    BleThreadDyn,
    BleThreadSta,
    BleZigbeeFfdDyn,
    BleZigbeeFfdSta,
    BleZigbeeRfdDyn,
    BleZigbeeRfdSta,
    Mac802154,
    Phy802154,
    ThreadFtd,
    ThreadMtd,
    ThreadRcp,
    ZigbeeFfd,
    ZigbeeRfd,
}

#[derive(Debug)]
pub struct TabWirelessStack {
    fw_selected: WirelessStackFile,
    serial_available_port: Vec<String>,
    serial_selected: Option<String>,
    log_widget: LogWidget,
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
                    Message::WirelessStack(TabWirelessStackMessage::StackSelected(x))
                })
                .width(Length::Fill)
            ),
            grid_row!(
                "Firmware file",
                pick_list(
                    &self.serial_available_port[..],
                    self.serial_selected.as_ref(),
                    |x| Message::WirelessStack(TabWirelessStackMessage::SerialSelected(x))
                )
                .width(Length::Fill)
            ),
        )
        .width(Length::Fill)
        .column_spacing(8)
        .row_spacing(8)
        .column_widths(&[Length::Shrink, Length::Fill]);

        let start_button = button(
            text("Flash the stack")
                .shaping(text::Shaping::Advanced)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .on_press(Message::WirelessStack(
            TabWirelessStackMessage::StartProcess,
        ))
        .width(Length::Fill);

        let log = container(self.log_widget.view())
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
            main_col
        };

        layout.into()
    }

    pub fn update(&mut self, message: TabWirelessStackMessage) -> Task<Message> {
        match message {
            TabWirelessStackMessage::StackSelected(file) => self.fw_selected = file,
            TabWirelessStackMessage::SerialSelected(serial) => self.serial_selected = Some(serial),

            TabWirelessStackMessage::StartProcess => {
                self.log_widget
                    .push(LogType::Info("Start flashing...".to_string()));
                self.is_readonly = true;
            }
        }

        Task::none()
    }
}

impl Default for TabWirelessStack {
    fn default() -> Self {
        Self {
            fw_selected: Default::default(),
            serial_available_port: Default::default(),
            serial_selected: Default::default(),
            log_widget: Default::default(),
            is_readonly: false,
        }
    }
}

impl std::fmt::Display for WirelessStackFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            WirelessStackFile::BleHciAdvScan => "BLE HCI AdvScan",
            WirelessStackFile::BleHciExt => "BLE HCI Layer extended",
            WirelessStackFile::BleHci => "BLE HCI Layer",
            WirelessStackFile::BleMac => "BLE Mac 802.15.4",
            WirelessStackFile::BleStackFullExt => "BLE Stack full extended",
            WirelessStackFile::BleStackFull => "BLE Stack full",
            WirelessStackFile::BleStackLight => "BLE Stack light",
            WirelessStackFile::BleThreadDyn => "BLE Thread dynamic",
            WirelessStackFile::BleThreadSta => "BLE Thread static",
            WirelessStackFile::BleZigbeeFfdDyn => "BLE Zigbee FFD dynamic",
            WirelessStackFile::BleZigbeeFfdSta => "BLE Zigbee FFD static",
            WirelessStackFile::BleZigbeeRfdDyn => "BLE Zigbee RFD dynamic",
            WirelessStackFile::BleZigbeeRfdSta => "BLE Zigbee RFD static",
            WirelessStackFile::Mac802154 => "Mac 802.15.4",
            WirelessStackFile::Phy802154 => "Phy 802.15.4",
            WirelessStackFile::ThreadFtd => "Thread FTD",
            WirelessStackFile::ThreadMtd => "Thread MTD",
            WirelessStackFile::ThreadRcp => "Thread RCP",
            WirelessStackFile::ZigbeeFfd => "Zigbee FFD",
            WirelessStackFile::ZigbeeRfd => "Zigbee RFD",
        })
    }
}
