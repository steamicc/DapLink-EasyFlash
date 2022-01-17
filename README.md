# Simple DAPLink tool

This tools is for internal usage, we used it to load daplink on STM32L475, to replace ST-LINK.

## Requirements
  - OpenOCD: `sudo apt install openocd`
  - Python3: `sudo apt install python3`
  - python3-tk: `sudo apt install python3-tk`

_All in one_ : `sudo apt install openocd python3 python3-tk` 

## Dev. Dependencies
_Just in case you wouldn't use the virtual env_
  - [PySimpleGUI](https://pysimplegui.readthedocs.io/en/latest/): `pip install pysimplegui`
  - virtual-env: `pip install virtualenv` _(optionnal)_
  - [psutil](https://psutil.readthedocs.io/en/latest/): `pip install psutil`

Enter in the virtual env before start changing anything ;)

## Usage

The program offer a simple GUI. to select the bin that will be flashed on the STM32F1x, and automatically apply the process.

  1. Launch the script `start.sh` (Linux) ~~or `start.ps1` (Windows)~~ TODO
  2. Select the files
     1. Select the bootloader binary to flash (e.g: `stm32f103xb_bl.bin`)
     2. Select the firmware binary to flash (e.g: `stm32f103xb_stm32l475vg_if.bin`)
     3. Select the program binary to flash _(optionnal)_
  3. Set the mount point name
     1. For "Maintenance", after bootloader was flashed (e.g: `MAINTENANCE`)
     2. For "Programming", after firmware was flashed (e.g: `DIS_L4IOT`, `DAPLINK`, ...)
  4. Define the timeout mount point waiting (e.g: `10000`), in milliseconds (1000 milliseconds = 1 second)
  5. Push the "Start" button.

**Pro tips**: All inputs are saved for the next time you will open the tool !


## Process
  * Unlock the RDP of the STM32F1x (if needed)
  * Mass erase flash
  * Write bootloader
  * Write firmware
  * Write test program _(optionnal)_
