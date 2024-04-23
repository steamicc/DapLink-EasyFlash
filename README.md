# Simple DAPLink tool

- [Simple DAPLink tool](#simple-daplink-tool)
  - [Introduction](#introduction)
  - [Requirements](#requirements)
    - [:computer: System](#computer-system)
      - [Linux](#linux)
      - [Windows](#windows)
      - [MacOs](#macos)
    - [:snake: Python](#snake-python)
    - [:floppy\_disk: Bootloader \& Firmware](#floppy_disk-bootloader--firmware)
  - [Usage](#usage)
    - [:electric\_plug: Hardware](#electric_plug-hardware)
      - [STM32 Disco L475 IoTNode](#stm32-disco-l475-iotnode)
        - [STLink V2 (or clones)](#stlink-v2-or-clones)
        - [Black Magic Probe (V2.1 in this picture)](#black-magic-probe-v21-in-this-picture)
        - [Nucleo's STLink](#nucleos-stlink)
      - [STM32 Nucleo WB55](#stm32-nucleo-wb55)
    - [:computer: Software](#computer-software)
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

![](doc/screenshot.png)



## Requirements

### :computer: System
#### Linux
 - [OpenOCD](https://openocd.org/): `sudo apt install openocd`
 - Python 3.x: `sudo apt install python3`
 - Python3 pip: `sudo apt install python3-pip`

:bulb: All in one: `sudo apt install openocd python3 python3-pip` 

#### Windows
  - [OpenOCD](https://openocd.org/): 
      - Download lastest release from [https://github.com/openocd-org/openocd/releases/latest](https://github.com/openocd-org/openocd/releases/latest)  
      - add the `bin` folder to your [path](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/) (e.g `C:/openOCD/bin`).
  - Python 3.x: Download it from [https://www.python.org/downloads/windows/](https://www.python.org/downloads/windows/)
  - Python3 pip: If not installed with Python, here are the instructions: [https://packaging.python.org/en/latest/tutorials/installing-packages/#requirements-for-installing-packages](https://packaging.python.org/en/latest/tutorials/installing-packages/#requirements-for-installing-packages)

#### MacOs
  Untested, but certainly identical to Linux.
  
### :snake: Python
  - [virtual-env](https://docs.python-guide.org/dev/virtualenvs/#lower-level-virtualenv): `pip install virtualenv` _(optionnal)_
  - [Tkinter](https://docs.python.org/fr/3/library/tkinter.html): `pip install tk` 
  - [psutil](https://psutil.readthedocs.io/en/latest/): `pip install psutil`

:bulb: You can install everything (virtual env include), with the `install.sh` script  (Linux only).

### :floppy_disk: Bootloader & Firmware
DapLink bootloaders and firmwares can be found at [https://github.com/letssteam/DAPLink/releases](https://github.com/letssteam/DAPLink/releases)

## Usage

### :electric_plug: Hardware
To allow the program flash the DapLink bootloader, the DapLink firmware, then the test program (optional), it's important to wiring you board.

**Note**: Connect the target (STM32 Disco L475 IoTNode, STM32 Nucleo WB55, ...) to your computer **after** wiring and connecting the probe to your computer

#### STM32 Disco L475 IoTNode
Here are somes schematics, to show you how to plug probes to the board.

##### STLink V2 (or clones)
![](doc/wiring_l475_stlinkv2.png "Wirring with the STLink V2 (or clones)")

##### Black Magic Probe (V2.1 in this picture)
![](doc/wiring_l475_bmp.png "Wirring with the Black Magick probe (V2.1 in the picture)")

##### Nucleo's STLink  
:warning: Remove both jumper `CN2` (orange rectangle) before flashing anything. Then **replace** them when you are finished.
![](doc/wiring_l475_nucleo.png "Wirring this the nucleo's STLink")

#### STM32 Nucleo WB55
_Soon..._ ;)

### :computer: Software
The program offer a simple GUI interface to select the bootload and the firmware that will be flashed on the STM32F1x.

  1. Launch the script `easy_daplink.py` file (e.g `python3 easy_daplink.py`)  
      _If you are using a virtual env activate it with `source venv/bin/activate`, or you can start the program with `start_venv.sh` (Linux only)_
  2. Select the `script` folder of OpenOCD (e.g On Linux : `/usr/share/openocd/scripts/`)
  3. Select files you downloaded from [requirements](#floppy_disk-bootloader--firmware)
     1. The bootloader binary file to flash (e.g: `stm32f103xb_bl.bin`)
     2. The firmware binary file to flash (e.g: `stm32f103xb_stm32l475vg_if.bin`)
     3. _(optionnal)_ The program binary file to flash (you can find test program in `test bin` folder)
  4. Set the mount point name
     1. For "Maintenance", after bootloader was flashed (e.g: `MAINTENANCE`)
     2. For "Programming", after firmware was flashed (e.g: `DIS_L4IOT`, `DAPLINK`, ...)
  5. Define the timeout mount point waiting (e.g: `10000`), in milliseconds (1000 milliseconds = 1 second)
  6. Push the "Start" button.

:bulb: Pro tips: All inputs are saved for the next time you will open the tool !


## Test files
In the `test bin` folder, you can find some simple programs for targets. 

### `test-l475.bin`
It blinks the LEDs, `LD1` and `LD2`, in two different patterns.  
![](doc/test_l475.gif)

### `test-wb55.bin`
Toggles between high and low states of pins `PC10` and `PC12`.  (The video is the result on the STeaMi board).  
![](doc/test_steami.gif)