# DapLink - EasyFlash

English - [Français](README_fr.md)

- [DapLink - EasyFlash](#daplink---easyflash)
  - [Introduction](#introduction)
    - [DapLink](#daplink)
    - [Stack Wireless](#stack-wireless)
  - [Requirements](#requirements)
    - [:computer: System](#computer-system)
    - [:floppy\_disk: Bootloader \& Firmware](#floppy_disk-bootloader--firmware)
  - [Usage](#usage)
    - [:electric\_plug: Probe wiring (DapLink)](#electric_plug-probe-wiring-daplink)
      - [STeaMi](#steami)
      - [STM32 Disco L475 IoTNode](#stm32-disco-l475-iotnode)
      - [STM32 Nucleo WB55](#stm32-nucleo-wb55)
    - [:computer: Software](#computer-software)
    - [DapLink](#daplink-1)
    - [Wireless Stack](#wireless-stack)
    - [:crab: Run from sources](#crab-run-from-sources)
  - [Test files](#test-files)
    - [`test-l475.bin`](#test-l475bin)
    - [`test-wb55.bin`](#test-wb55bin)
  - [Stack details](#stack-details)


## Introduction
This tool is for internal use, we use it (1) to load daplink on certain targets (STM32L475, STM32WB55, ...), replacing ST-LINK and (2) to flash a wireless stack (BLE, Thread, Zigbee, ...) on the STM32WB5xxG co-processor.

### DapLink
With OpenOCD, the program steps are :
  1. Unlock the RDP of the STM32F103xB
  2. Mass erase flash
  3. Flash bootloader
  4. Send firmware
  5. _(optionnal)_ Send test program 

### Stack Wireless
The stack is flashed to the co-processor using the [FUS](https://wiki.st.com/stm32mcu/wiki/Connectivity:STM32WB_FUS) and a program called `operator`, which acts as a relay between high-level comands and the FUS. The `operator` is precompiled, but **is not** the official ST Microelectronics version, the sources are available [here](https://github.com/steamicc/codal-steami-samples/tree/main/samples/Peripherals/FUS_WS_Operator) ([https://github.com/steamicc/codal-steami-samples/tree/main/samples/Peripherals/FUS_WS_Operator](https://github.com/steamicc/codal-steami-samples/tree/main/samples/Peripherals/FUS_WS_Operator)).


![screenshot](doc/screenshot.png)

_Appearance may vary depending on your OS configuration._

## Requirements

### :computer: System
To avoid installing OpenOCD on your computer, we recommend using the pre-built version maintained by [XPack](https://github.com/xpack-dev-tools/openocd-xpack/releases/tag/v0.12.0-4). The tested and compatible version is `v0.12.0-4`.

:bulb: The archives available in [releases](https://github.com/steamicc/DapLink-EasyFlash/releases) already contains all the files placed where they need to be.

:warning: **Mac OS X** is not yet supported, but PRs are welcome :wink:

### :floppy_disk: Bootloader & Firmware
- Bootloader
  - STM32F103xB: [https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_bl.bin](https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_bl.bin)

- Firmware
  - STM32L475VG: [https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_stm32l475vg_if.bin
](https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_stm32l475vg_if.bin
)  
  - STM32WB55RG: [https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_stm32wb55rg_if.bin
](https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_stm32wb55rg_if.bin
)  
  - STeaMi: [https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_steami32_if.bin
](https://github.com/letssteam/DAPLink/releases/latest/download/stm32f103xb_steami32_if.bin
)

:bulb: Older releases are available here: [https://github.com/letssteam/DAPLink/releases](https://github.com/letssteam/DAPLink/releases)

:warning: For other targets, please go directly to the [DapLink](https://github.com/ARMmbed/DAPLink) GitHub 
## Usage

### :electric_plug: Probe wiring (DapLink)
To enable the program to flash the bootloader, firmware and then the test program, the board must be wired to a probe (ST-Link, Black magic probe, etc.), and connected to the computer (using a micro-USB cable).

:warning: **Connect the target** (STM32 Disco L475 IoTNode, STM32 Nucleo WB55, ...) to your computer **after** wiring and **connecting the probe** to your computer

#### STeaMi
![](doc/wiring_stlink_steami.png "Wiring STeaMi")

#### STM32 Disco L475 IoTNode
![](doc/wiring_l475_stlinkv2.png "Wiring with the STLink V2 (or clones)")

#### STM32 Nucleo WB55
![](doc/wiring_stlink_nucleo.png "Wiring Nucleo WB55")


### :computer: Software
The program offer a simple GUI interface to select the bootload and the firmware that will be flashed on the STM32F1x.

### DapLink
  1. Select files you downloaded from [requirements](#floppy_disk-bootloader--firmware)
     1. The bootloader binary file to flash (e.g: `stm32f103xb_bl.bin`)
     2. The firmware binary file to flash (e.g: `stm32f103xb_stm32l475vg_if.bin`)
     3. _(optionnal)_ The program binary file to flash (you can find test program in `test bin` folder)
  2. Set the target mount point name (e.g: `DIS_L4IOT`, `DAPLINK`, `STEAMI`...)
  3. Define the timeout mount point waiting (e.g: `10`), in seconds
  4. Push the "Start" button.

### Wireless Stack
 1. Select the board serial port
 2. Choose a stack. A description of the stacks is available [below](#stack-details)
 3. Press the “Start” button

:bulb: Pro tips: All inputs are saved for the next time you will open the tool !


### :crab: Run from sources
1. Install [rust](https://www.rust-lang.org/tools/install)
2. Install `openocd`
3. Clone or download this repository `git clone https://github.com/steamicc/DapLink-EasyFlash.git`
4. Enter in project folder `cd DapLink-EasyFlash`
5. Run `cargo run` from the project root.

## Test files
In the `test bin` folder, you can find some simple programs for targets. 

### `test-l475.bin`
It blinks the LEDs, `LD1` and `LD2`, in two different patterns.  
![](doc/test_l475.gif)

### `test-wb55.bin`
Toggles between high and low states of pins `PC10` and `PC12`.  (The video is the result on the STeaMi board).  
![](doc/test_steami.gif)

## Stack details
| Firmware | Description | STM32WB5xxG (1M) | STM32WB5xxY (640K) | STM32WB5xxE (512K) | STM32WB5xxC (256K) |
| --- | --- | --- | --- | --- | --- |
| BLE HCI AdvScan | *   To be used for advertising and scanning through HCI interface<br>*   BT SIG Certification listing : [Declaration ID D042213 / QDID 160726](https://launchstudio.bluetooth.com/ListingDetails/120678)<br>*   HCI Layer only mode, layers supported : Link Layer, HCI | ✅   | ✅   | ✅   | ✅   |
| BLE LLD | *   BLE LLD (Low Level Driver) Radio Transparent firmware<br>*   To be used for direct access on BLE LLD features and API | ✅   | ✅   | ✅   | ✅   |
| BLE Stack full | *   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676)<br>*   Full BLE Stack, layers supported : Link Layer, HCI, L2CAP, ATT, SM, GAP and GATT database<br>    *   Following features are kept (based on Basic stack library compared to previous deliveries):<br>        *   GAP peripheral, central (Master up to 8 links/Slave up to 8 links/all combinations in between)<br>        *   GATT server, client<br>        *   Data length extension<br>        *   2Mbit PHY / PHY update<br>        *   Privacy<br>        *   White list<br>        *   Legacy Pairing, LE secure connections<br>        *   Direct Test Mode<br>        *   HCI interface (full, like stm32wb5x\_BLE\_HCILayer\_fw.bin)<br>    *   Following features are removed:<br>        *   **L2Cap Connection - oriented channels support (IP over BLE enabler)**<br>        *   **Channel selection #2 (under application flag selection)**<br>        *   **Some HCI interface features (won’t be able to process through HCI interface)** | ✅   | ✅   | ✅   | ✅   |
| BLE Stack full extended | *   BT SIG Certification listing (1) : [Declaration ID D060553 / QDID 182505](https://launchstudio.bluetooth.com/ListingDetails/146231)<br>*   BT SIG Certification listing (2) : [Declaration ID D063069 / QDID 201968](https://launchstudio.bluetooth.com/ListingDetails/170086)<br>*   BT SIG Certification listing (3) : [Declaration ID D063070 / QDID 216169](https://launchstudio.bluetooth.com/ListingDetails/186628)<br>*   Full BLE Stack extended, layers supported : Link Layer, HCI, L2CAP, ATT, SM, GAP and GATT database<br>    *   Following features are kept:<br>        *   GAP peripheral, central (Master up to 8 links/Slave up to 8 links/all combinations in between)<br>        *   GATT server, client<br>        *   Data length extension<br>        *   2Mbit PHY / PHY update<br>        *   Privacy<br>        *   White list<br>        *   Legacy Pairing, LE secure connections<br>        *   HCI interface (full, like stm32wb5x\_BLE\_HCILayer\_fw.bin)<br>        *   Direct Test Mode<br>        *   L2CAP connection oriented channels support (IP over BLE enabler)<br>        *   Channel selection #2 (under application flag selection)<br>        *   BLE Extended advertising (under application SHCI\_C2\_BLE\_INIT\_OPTIONS\_EXT\_ADV flag selection with following limitations on currently supported configurations as (max sets number, max advertising data length) equal to \[(1,1650),(2,1650),(3,1650),(4,1035),(5,621),(6,414),(7,207),(8,207)\] such as both parameters are compliant with allocated Total memory computed with BLE\_EXT\_ADV\_BUFFER\_SIZE based on Max Extended advertising configuration.<br>        *   BLE GATT caching supported (certified BLE 5.3)<br>        *   BLE Enhanced ATT supported (certified BLE 5.3)<br>*   **Warning**: To use this binary, it is necessary to adapt the scatter file in the BLE applications as:<br>    *   The RAM\_A shared range shall be reduced to memory range \[0x20030000:0x200307FF\]<br>    *   The Mail-box buffers(MB\_MEM1, MB\_MEM2) shall be located in RAM\_B shared defined in memory range \[0x20038000:0x2003A7FF\]<br>    *   The RAM\_B shared shall be added to Total\_RAM\_region | ✅   | ✅   | ✅   | ✅   |
| BLE Stack light | *   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676)<br>*   Full BLE Stack, layers supported : Link Layer, HCI, L2CAP, ATT, SM, GAP and GATT database<br>*   Wireless Ble stack Light configuration – Slave Only<br>    *   Following features are kept:<br>        *   GAP peripheral only (LL Slave up to 4 links)<br>        *   GATT server<br>        *   Data length extension<br>        *   2Mbit PHY / PHY update<br>        *   Privacy<br>        *   White list<br>        *   Legacy Pairing, LE secure connections<br>        *   Direct Test Mode<br>        *   HCI interface (reduced)<br>        *   Channel selection #2 \[CSA2\] feature added<br>        *   **Additional beacon**<br>*   Following features are removed:<br>    *   BLE “Slave Only” stack implies that with this stack configuration, STM32WB is not able to scan and request a BLE connection.<br>    *   It will just advertise, and accept incoming connection request from other master devices (e.g. Smartphone).<br>    *   While with the “full feature” BLE stack, STM32WB5xx is able to support both master and slave roles on different links (with the limitation of max 8 links in parallel). | ✅   | ✅   | ✅   | ✅   |
| BLE HCILayer | *   BT SIG Certification listing : [Declaration ID D042213 / QDID 160726](https://launchstudio.bluetooth.com/ListingDetails/120678)<br>*   HCI Layer only mode, layers supported : Link Layer, HCI with Direct Test Mode | ✅   | ✅   | ✅   | ✅   |
| BLE HCILayer extended | *   BT SIG Certification listing (1) : [Declaration ID D060553 / QDID 182505](https://launchstudio.bluetooth.com/ListingDetails/146231)<br>*   BT SIG Certification listing (2) : [Declaration ID D063069 / QDID 201968](https://launchstudio.bluetooth.com/ListingDetails/170086)<br>*   HCI Layer only mode extended, layers supported : Link Layer, HCI with Direct Test Mode<br>*   BLE Extended advertising (under application SHCI\_C2\_BLE\_INIT\_OPTIONS\_EXT\_ADV flag selection with following limitations on currently supported configurations as (max sets number, max advertising data length) equal to \[(1,1650),(2,1650),(3,1650),(4,1035),(5,621),(6,414),(7,207),(8,207)\] such as both parameters are compliant with allocated Total memory computed with BLE\_EXT\_ADV\_BUFFER\_SIZE based on Max Extended advertising configuration. | ✅   | ✅   | ✅   | ✅   |
| Thread FTD | *   Full Thread Device v1.3 ready<br>*   To be used for Leader / Router / End Device Thread role (full features excepting Border Router) | ✅   | ✅   | ✅   | ❌   |
| Thread MTD | *   Minimal Thread Device v1.3 ready<br>*   To be used for End Device and Sleepy End Device Thread role | ✅   | ✅   | ✅   | ❌   |
| Thread RCP | *   OpenThread Radio Co-Processor (RCP)<br>*   To be used for Thread Border Router setup.<br>*   Application layer and OpenThread core on the host processor, minimal OpenThread MAC on the 802.15.4 SoC.<br>*   Communication between the RCP and the host processor is managed by OpenThread Daemon through an UART interface over the Spinel protocol. | ✅   | ✅   | ✅   | ✅   |
| BLE Thread static | *   Static Concurrent Mode BLE Thread<br>*   Supports Full BLE Stack and Full Thread Device v1.3 ready<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ❌   | ❌   |
| BLE Thread dynamic | *   Dynamic Concurrent Mode BLE Thread<br>*   Supports Full BLE Stack and Full Thread Device v1.3 ready<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ❌   | ❌   |
| Mac 802\_15\_4 | *   MAC API is based on latest official [IEEE Std 802.15.4-2011](http://grouper.ieee.org/groups/802/15/pub/Download.html)<br>*   To be used for MAC FFD and RFD devices | ✅   | ✅   | ✅   | ✅   |
| Phy 802\_15\_4 | *   802.15.4 Features exposed on application side<br>*   Reduced number of commands called from application side to manage 802.15.4 API<br>*   Not a Transparent mode, 802.15.4 API not deployed on application side<br>*   Can to used with STM32CubeMonitor-RF application or dedicated M4 Application. | ✅   | ✅   | ✅   | ✅   |
| Zigbee RFD | *   Zigbee Reduced Function Device<br>*   Zigbee Compliant Platform ready<br>*   To be used for End Device Zigbee role | ✅   | ✅   | ✅   | ❌   |
| Zigbee FFD | *   Zigbee Compliant Platform ready<br>*   Supports Full Function Device (FFD) | ✅   | ✅   | ✅   | ❌   |
| BLE Mac 802\_15\_4 | *   Static Concurrent Mode BLE MAC 802.15.4.<br>*   Supports Full BLE Stack and MAC 802.15.4 API based on latest official [IEEE Std 802.15.4-2011](http://grouper.ieee.org/groups/802/15/pub/Download.html)<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ✅   | ✅   |
| BLE Zigbee FFD static | *   Static Concurrent Mode BLE Zigbee FFD.<br>*   Supports Full BLE Stack and Zigbee FFD(Full Function Device) Compliant Platform ready.<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ✅   | ❌   |
| BLE Zigbee RFD static | *   Static Concurrent Mode BLE Zigbee RFD.<br>*   Supports Full BLE Stack and Zigbee RFD(Reduced Function Device) Compliant Platform ready.<br>*   Optimized for Power consumption.<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ✅   | ❌   |
| BLE Zigbee FFD dynamic | *   Dynamic Concurrent Mode BLE Zigbee FFD.<br>*   Supports Full BLE Stack and Zigbee FFD(Full Function Device) Compliant Platform ready.<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ✅   | ❌   |
| BLE Zigbee RFD dynamic | *   Dynamic Concurrent Mode BLE Zigbee RFD.<br>*   Supports Full BLE Stack and Zigbee RFD(Reduced Function Device) Compliant Platform ready.<br>*   Optimized for Power consumption.<br>*   BT SIG Certification listing : [Declaration ID D042164 / QDID 160724](https://launchstudio.bluetooth.com/ListingDetails/120676) | ✅   | ✅   | ✅   | ❌   |