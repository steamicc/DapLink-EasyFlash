#include "FUS_WS_Operator.h"

#include <cstdio>
#include <string>

#include "HCI_SharedMemory.h"
#include "shci.h"
#include "stm32_def.h"

using namespace std;
using namespace codal;

struct Fus_WS_Status {
    union {
        struct {
            uint8_t current_wireless_stack : 8;
            uint8_t last_wireless_stack_state : 8;
            uint8_t last_fus_active_state : 8;
            uint8_t RSVD : 8;
        };
        uint32_t status;
    };
};

bool click_button(codal::STM32Pin& btn)
{
    if (btn.getDigitalValue() == 0) {
        while (btn.getDigitalValue() == 0);

        return true;
    }

    return false;
}

void FUS_WS_Operator(STeaMi& steami)
{
    // Start CRC clock for FUS
    __HAL_RCC_CRC_CLK_ENABLE();

    steami.serial.init(115'200);
    HCI_SharedMemory hci;
    hci.init();

    LL_PWR_EnableBootC2();

    uint32_t base_address                  = *((uint32_t*)0x20030000);
    uint32_t offset_version                = 0xC;
    uint32_t offsset_Fus_WS_Status         = 0x4;
    uint32_t offset_wireless_stack_version = 0x14;

    while (true) {
        ManagedString str = steami.serial.readUntil("\n", ASYNC);

        if (str.length() > 0) {
            string command(str.toCharArray(), str.length());
            for (auto& c : command) c = toupper(c);

            if (command == "DELETE") {
                printf(R"({"command ": "Delete", "status": %u})", SHCI_C2_FUS_FwDelete());
                printf("\n");
            }

            if (command == "RESET") {
                target_reset();
            }

            if (command == "START") {
                printf(R"({"command": "Start", "status": %u})", SHCI_C2_FUS_StartWs());
                printf("\n");
            }

            if (command == "STATUS") {
                Fus_WS_Status fus_ws_status;
                SHCI_FUS_GetState_ErrorCode_t error_get_state;
                uint8_t status = SHCI_C2_FUS_GetState(&error_get_state);

                if (status != 0) {
                    printf(R"({"command": "Status", "status": %u, "error": %u})", status, error_get_state);
                    printf("\n");
                    continue;
                }

                fus_ws_status.status = *((uint32_t*)(base_address + offsset_Fus_WS_Status));

                printf(
                    R"({"command": "Status", "status": %u, "last_fus_status": %u, "last_ws_status": %u, "current_ws": %u})",
                    status, fus_ws_status.last_fus_active_state, fus_ws_status.last_wireless_stack_state,
                    fus_ws_status.current_wireless_stack);
                printf("\n");
            }

            if (command == "UPGRADE") {
                uint8_t status = 0xFF;
                SHCI_FUS_GetState_ErrorCode_t error_get_state;
                printf(R"({"command": "Upgrade", "status": %u})", SHCI_C2_FUS_FwUpgrade(0, 0));
                printf("\n");
                while (status != 0) {
                    status = SHCI_C2_FUS_GetState(&error_get_state);
                    printf(R"#({"command": "Upgrade(status)", "status": %u, "error": %u})#", status, error_get_state);
                    printf("\n");
                    target_wait(100);
                }
            }

            if (command == "VERSION") {
                WirelessFwInfo_t p_wireless_info;
                SHCI_GetWirelessFwInfo(&p_wireless_info);
                printf(
                    R"({"command": "Version", "status": 0, "fus_version": %u, "copro_fw_version": "%d.%d.%d", "ws_version": %u})",
                    *((uint32_t*)(base_address + offset_version)), p_wireless_info.VersionMajor,
                    p_wireless_info.VersionMinor, p_wireless_info.VersionSub,
                    *((uint32_t*)(base_address + offset_wireless_stack_version)));
                printf("\n");
            }

            if (command == "HELP") {
                printf(
                    "Available commands (case insensitive):\n"
                    "\tDELETE\tDelete WS firmware\n"
                    "\tRESET\tReset target\n"
                    "\tSTART\tStart WS\n"
                    "\tSTATUS\tCall FUS GetState and read head of Device Information table\n"
                    "\tUPGRADE\tCall FUS Firmware Upgrade\n"
                    "\tVERSION\tGet FUS & WS versions\n"
                    "\n\tHELP\tShow this help\n");
            }
        }
    }
}
