# Simple DAPLink tool

- [Simple DAPLink tool](#simple-daplink-tool)
  - [Introduction](#introduction)
  - [Requirements](#requirements)
    - [:computer: System](#computer-system)
      - [OpenOCD](#openocd)
        - [Linux (Ubuntu)](#linux-ubuntu)
        - [Windows](#windows)
        - [MacOs](#macos)
    - [:floppy\_disk: Bootloader \& Firmware](#floppy_disk-bootloader--firmware)
  - [Usage](#usage)
    - [:electric\_plug: Hardware](#electric_plug-hardware)
      - [STM32 Disco L475 IoTNode](#stm32-disco-l475-iotnode)
        - [STLink V2 (or clones)](#stlink-v2-or-clones)
        - [Black Magic Probe (V2.1 in this picture)](#black-magic-probe-v21-in-this-picture)
        - [Nucleo's STLink](#nucleos-stlink)
      - [STM32 Nucleo WB55](#stm32-nucleo-wb55)
      - [STeaMi](#steami)
    - [:computer: Software](#computer-software)
    - [:crab: Run from sources](#crab-run-from-sources)
  - [Test files](#test-files)
    - [`test-l475.bin`](#test-l475bin)
    - [`test-wb55.bin`](#test-wb55bin)

## Introduction
This tools is for internal usage, we use it to load daplink on target (STM32L475, STM32WB55, ...), to replace ST-LINK.

With OpenOCD, the program steps are :
  1. Unlock the RDP of the STM32F1x (if needed)
  2. Mass erase flash
  3. Flash bootloader
  4. Send firmware
  5. _(optionnal)_ Send test program 

![screenshot](doc/screenshot.png)

_Appearance may vary depending on your OS configuration._



## Requirements

### :computer: System
#### [OpenOCD](https://openocd.org/)
##### Linux (Ubuntu)
OpenOCD should be available via apt `sudo apt install openocd`. 

##### Windows
To avoid having to declare environment variables, we recommend obtaining a pre-built version of OpenOCD ([XPack](https://github.com/xpack-dev-tools/openocd-xpack/releases/) for example) and organizing files as follows:
   - OpenOCD **executable and DLLs** in the **same folder** as the DapLink-EasyFlash executable
   - OpenOCD **scripts** folder in the **same folder** as the DapLink-EasyFlash executable

💡The windows zip archive available in [releases](https://github.com/letssteam/DapLink-EasyFlash/releases) already contains all the files placed where they need to be.

##### MacOs
Untested, but certainly identical to Linux.
  

### :floppy_disk: Bootloader & Firmware
DapLink bootloaders and firmwares can be found at [https://github.com/letssteam/DAPLink/releases](https://github.com/letssteam/DAPLink/releases)

## Usage

### :electric_plug: Hardware
To allow the program flash the DapLink bootloader, the DapLink firmware, then the test program (optional), it's important to wiring you board.

:warning: **Connect the target** (STM32 Disco L475 IoTNode, STM32 Nucleo WB55, ...) to your computer **after** wiring and **connecting the probe** to your computer

#### STM32 Disco L475 IoTNode
Here are somes schematics, to show you how to plug probes to the board.

##### STLink V2 (or clones)
![](doc/wiring_l475_stlinkv2.png "Wiring with the STLink V2 (or clones)")

##### Black Magic Probe (V2.1 in this picture)
![](doc/wiring_l475_bmp.png "Wiring with the Black Magick probe (V2.1 in the picture)")

##### Nucleo's STLink  
:warning: Remove both jumper `CN2` (orange rectangle) before flashing anything. Then **replace** them when you are finished.
![](doc/wiring_l475_nucleo.png "Wiring this the nucleo's STLink")

#### STM32 Nucleo WB55
![](doc/wiring_stlink_nucleo.png "Wiring Nucleo WB55")

#### STeaMi
![](doc/wiring_stlink_steami.png "Wiring STeaMi")

### :computer: Software
The program offer a simple GUI interface to select the bootload and the firmware that will be flashed on the STM32F1x.


  1. Select files you downloaded from [requirements](#floppy_disk-bootloader--firmware)
     1. The bootloader binary file to flash (e.g: `stm32f103xb_bl.bin`)
     2. The firmware binary file to flash (e.g: `stm32f103xb_stm32l475vg_if.bin`)
     3. _(optionnal)_ The program binary file to flash (you can find test program in `test bin` folder)
  2. Set the target mount point name (e.g: `DIS_L4IOT`, `DAPLINK`, `STEAMI`...)
  3. Define the timeout mount point waiting (e.g: `10`), in seconds
  4. Push the "Start" button.

:bulb: Pro tips: All inputs are saved for the next time you will open the tool !


### :crab: Run from sources
1. Install [rust](https://www.rust-lang.org/tools/install)
2. Clone or download this repository
3. Run `cargo run` from the project root.


## Test files
In the `test bin` folder, you can find some simple programs for targets. 

### `test-l475.bin`
It blinks the LEDs, `LD1` and `LD2`, in two different patterns.  
![](doc/test_l475.gif)

### `test-wb55.bin`
Toggles between high and low states of pins `PC10` and `PC12`.  (The video is the result on the STeaMi board).  
![](doc/test_steami.gif)