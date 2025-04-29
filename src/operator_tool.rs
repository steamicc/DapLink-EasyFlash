use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OperatorStatusResult {
    pub status: u32,
    pub last_fus_status: u32,
    pub last_ws_status: u32,
    pub current_ws: u32,
}

#[allow(unused)]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct OperatorVersionResult {
    pub status: u32,
    pub fus_version: u32,
    pub copro_fw_version: String,
    pub ws_version: u32,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct OperatorResult {
    pub status: u32,
    pub error: Option<u32>,
}

pub fn operator_error_string(error_code: u32) -> &'static str {
    match error_code{
        0x00 => "FUS_STATE_NO_ERROR => No error occurred.",
        0x01 => "FUS_STATE_IMG_NOT_FOUND => Firmware/FUS upgrade requested but no image found. (such as image header corrupted or flash memory corrupted)",
        0x02 => "FUS_SATE_IMC_CORRUPT => Firmware/FUS upgrade requested, image found, authentic but not integer (corruption on the data)",
        0x03 => "FUS_STATE_IMG_NOT_AUTHENTIC => Firmware/FUS upgrade requested, image found, but its signature is not valid (wrong signature, wrong signature header)",
        0x04 => "FUS_SATE_NO_ENOUGH_SPACE => Firmware/FUS upgrade requested, image found and authentic, but there is no enough space to install it due to the already installed image. Install the stack in a lower location then try again.",
        0x05 => "FUS_IMAGE_USRABORT => Operation aborted by user or power off occurred",
        0x06 => "FUS_IMAGE_ERSERROR => Flash Erase Error",
        0x07 => "FUS_IMAGE_WRTERROR => Flash Write Error",
        0x08 => "FUS_AUTH_TAG_ST_NOTFOUND => STMicroelectronics Authentication tag not found error in the image",
        0x09 => "FUS_AUTH_TAG_CUST_NOTFOUND => Customer Authentication tag not found in the image",
        0x0A => "FUS_AUTH_KEY_LOCKED => The key that the user tries to load is currently locked",
        0x11 => "FUS_FW_ROLLBACK_ERROR",
        _ => "Unknow code"
    }
}

pub fn upgrade_status_string(status_code: u32) -> &'static str {
    if status_code >= 0x30 {
        "FUS_STATE_SERVICE_ONGOING"
    } else if status_code >= 0x20 {
        "FUS_STATE_FUS_UPGRD_ONGOING"
    } else if status_code >= 0x10 {
        "FUS_STATE_FW_UPGRD_ONGOING"
    } else if status_code == 0x00 {
        "FUS_STATE_IDLE"
    } else {
        "Unknown status code"
    }
}
