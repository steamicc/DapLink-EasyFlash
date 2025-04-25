#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WirelessStackFile {
    BleHciAdvScan,
    #[default]
    BleHciExt,
    BleHci,
    BleMac,
    BleLld,
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

pub enum FusFile {
    FusFor0_5_3,
    Fus1_2_0,
}

pub const fn fus_config(file: FusFile) -> &'static str {
    match file {
        FusFile::FusFor0_5_3 => "stm32wb5xxG_FUS_fw_for_fus_0_5_3.hex",
        FusFile::Fus1_2_0 => "stm32wb5xxG_FUS_fw.hex",
    }
}

pub const fn wireless_stack_config(file: WirelessStackFile) -> &'static str {
    match file {
        WirelessStackFile::BleHciExt => "stm32wb5xxG_BLE_HCILayer_extended_fw.hex",
        WirelessStackFile::BleHci => "stm32wb5xxG_BLE_HCILayer_fw.hex",
        WirelessStackFile::BleHciAdvScan => "stm32wb5xxG_BLE_HCI_AdvScan_fw.hex",
        WirelessStackFile::BleLld => "stm32wb5xxG_BLE_LLD_fw.hex",
        WirelessStackFile::BleMac => "stm32wb5xxG_BLE_Mac_802_15_4_fw.hex",
        WirelessStackFile::BleStackFullExt => "stm32wb5xxG_BLE_Stack_full_extended_fw.hex",
        WirelessStackFile::BleStackFull => "stm32wb5xxG_BLE_Stack_full_fw.hex",
        WirelessStackFile::BleStackLight => "stm32wb5xxG_BLE_Stack_light_fw.hex",
        WirelessStackFile::BleThreadDyn => "stm32wb5xxG_BLE_Thread_dynamic_fw.hex",
        WirelessStackFile::BleThreadSta => "stm32wb5xxG_BLE_Thread_static_fw.hex",
        WirelessStackFile::BleZigbeeFfdDyn => "stm32wb5xxG_BLE_Zigbee_FFD_dynamic_fw.hex",
        WirelessStackFile::BleZigbeeFfdSta => "stm32wb5xxG_BLE_Zigbee_FFD_static_fw.hex",
        WirelessStackFile::BleZigbeeRfdDyn => "stm32wb5xxG_BLE_Zigbee_RFD_dynamic_fw.hex",
        WirelessStackFile::BleZigbeeRfdSta => "stm32wb5xxG_BLE_Zigbee_RFD_static_fw.hex",
        WirelessStackFile::Mac802154 => "stm32wb5xxG_Mac_802_15_4_fw.hex",
        WirelessStackFile::Phy802154 => "stm32wb5xxG_Phy_802_15_4_fw.hex",
        WirelessStackFile::ThreadFtd => "stm32wb5xxG_Thread_FTD_fw.hex",
        WirelessStackFile::ThreadMtd => "stm32wb5xxG_Thread_MTD_fw.hex",
        WirelessStackFile::ThreadRcp => "stm32wb5xxG_Thread_RCP_fw.hex",
        WirelessStackFile::ZigbeeFfd => "stm32wb5xxG_Zigbee_FFD_fw.hex",
        WirelessStackFile::ZigbeeRfd => "stm32wb5xxG_Zigbee_RFD_fw.hex",
    }
}
