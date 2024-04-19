from datetime import datetime
from sys import stderr
import time
import os
import threading
import subprocess
import shutil
import psutil
import re

import PySimpleGUI as sg

from settings import Settings

SCRIPT_FOLDER = "-SCRIPT_FOLDER-"
BOOTLOADER = "-BOOTLOADER-"
FIRMWARE = "-FIRMWARE-"
PROGRAM = "-TEST_PROGRAM-"
START_BUTTON = "-START-"
TIMEOUT_MOUNT = "-TIMEOUT_MOUNT_POINT-"
MAINTENACE_MOUNT_NAME = "-MAINTENANCE_MOUNT_POINT-"
PROGRAM_MOUNT_NAME = "-PROGRAMMING_MOUNT_POINT-"

settings = Settings("settings.dat")
sg.theme("BlueMono")

layout = [
    [
        sg.Text("OpenOCD 'scripts' folder:"),
        sg.Input(enable_events=True, key=SCRIPT_FOLDER, default_text=settings.get_value_or_default(SCRIPT_FOLDER, "/usr/share/openocd/scripts/") ),
        sg.FolderBrowse(initial_folder=settings.get_value(SCRIPT_FOLDER))
    ],
    [
        sg.Text("Select the BOOTLOADER file:"),
        sg.Input(enable_events=True, key=BOOTLOADER, default_text=settings.get_value(BOOTLOADER)),
        sg.FileBrowse(file_types=(("Compatible files", "*.bin *.hex"), ("BIN files", "*.bin"), ("HEX files", "*.hex"), ("All Files", "*.* *"),), initial_folder=os.path.dirname(settings.get_value_or_default(BOOTLOADER, "")))
    ],
    [
        sg.Text("Select the FIRMWARE file:"),
        sg.Input(enable_events=True, key=FIRMWARE, default_text=settings.get_value(FIRMWARE)),
        sg.FileBrowse(file_types=(("Compatible files", "*.bin *.hex"), ("BIN files", "*.bin"), ("HEX files", "*.hex"), ("All Files", "*.* *"),), initial_folder=os.path.dirname(settings.get_value_or_default(FIRMWARE, "")))
    ],
    [
        sg.Text("Select the TEST PROGRAM file (skip if empty):"),
        sg.Input(enable_events=True, key=PROGRAM, default_text=settings.get_value(PROGRAM)),
        sg.FileBrowse(file_types=(("Compatible files", "*.bin *.hex"), ("BIN files", "*.bin"), ("HEX files", "*.hex"), ("All Files", "*.* *"),), initial_folder=os.path.dirname(settings.get_value_or_default(PROGRAM, "")))
    ],
    [
        sg.Text("\"MAINTENANCE\" mount point name: "),
        sg.Input(enable_events=True, key=MAINTENACE_MOUNT_NAME, default_text=settings.get_value_or_default(MAINTENACE_MOUNT_NAME, "MAINTENANCE") )
    ],
    [
        sg.Text("\"DAPLINK\" programming mount point name: "),
        sg.Input(enable_events=True, key=PROGRAM_MOUNT_NAME, default_text=settings.get_value_or_default(PROGRAM_MOUNT_NAME, "DIS_L4IOT" ) )
    ],
    [
        sg.Text("Timeout (in milliseconds) for mount points: "),
        sg.Input(enable_events=True, key=TIMEOUT_MOUNT, default_text=settings.get_value_or_default(TIMEOUT_MOUNT, "10000" ) )
    ],
    [
        sg.Button(button_text="Start !", enable_events=True, key=START_BUTTON, expand_x=True, disabled=True)
    ],
    [
        sg.Multiline(disabled=True, expand_x=True, autoscroll=True, size=(None, 30), key="-LOG-", font="monospace 8")
    ]
]

window = sg.Window(title="Simple DapLink board", layout=layout, margins=(32, 32))

def main():
    default_bg_input = sg.theme_input_background_color()
    running_thread = None

    while True:
        event, values = window.read(100)

        if event == sg.WIN_CLOSED or event == "Quit":
            break

        elif event == SCRIPT_FOLDER :
            if is_valid_dir(values[SCRIPT_FOLDER]):
                window[SCRIPT_FOLDER].update(background_color=default_bg_input)
                settings.set_value(SCRIPT_FOLDER, values[SCRIPT_FOLDER])
            else:
                window[SCRIPT_FOLDER].update(background_color="#DD5555")

        elif event == BOOTLOADER :
            if is_valid_file(values[BOOTLOADER]):
                window[BOOTLOADER].update(background_color=default_bg_input)
                settings.set_value(BOOTLOADER, values[BOOTLOADER])
            else:
                window[BOOTLOADER].update(background_color="#DD5555")

        elif event == FIRMWARE :
            if is_valid_file(values[FIRMWARE]):
                window[FIRMWARE].update(background_color=default_bg_input)
                settings.set_value(FIRMWARE, values[FIRMWARE])
            else:
                window[FIRMWARE].update(background_color="#DD5555")

        elif event == PROGRAM :
            if is_valid_file(values[PROGRAM]) or len(values[PROGRAM]) == 0:
                window[PROGRAM].update(background_color=default_bg_input)
                settings.set_value(PROGRAM, values[PROGRAM])
            else:
                window[PROGRAM].update(background_color="#DD5555")

        elif event == TIMEOUT_MOUNT :
            if is_valid_number(values[TIMEOUT_MOUNT]):
                window[TIMEOUT_MOUNT].update(background_color=default_bg_input)
                settings.set_value(TIMEOUT_MOUNT, values[TIMEOUT_MOUNT])
            else:
                window[TIMEOUT_MOUNT].update(background_color="#DD5555")

        elif event == MAINTENACE_MOUNT_NAME :
            if len(values[MAINTENACE_MOUNT_NAME]) > 0:
                window[MAINTENACE_MOUNT_NAME].update(background_color=default_bg_input)
                settings.set_value(MAINTENACE_MOUNT_NAME, values[MAINTENACE_MOUNT_NAME])
            else:
                window[MAINTENACE_MOUNT_NAME].update(background_color="#DD5555")

        elif event == PROGRAM_MOUNT_NAME :
                settings.set_value(PROGRAM_MOUNT_NAME, values[PROGRAM_MOUNT_NAME])

        elif event == START_BUTTON:
            running_thread = threading.Thread(target=openocd_procedure, args=(values,))
            running_thread.start()

        # elif event != None :
        #     print(event)
        
        update_start_button_state(values, running_thread != None)

        if running_thread != None and not running_thread.is_alive():
            running_thread = None


    window.close()



###################################################################
#### LOG FUNCTIONS #############################################
#############################################################

def log(msg, level, no_time=False):
    log = window["-LOG-"]

    if no_time:
        final_msg = msg
    else:
        final_msg = "[{}]    {}".format(datetime.now().strftime("%H:%M:%S.%f"), msg)

    if level == "error":
        log.print(final_msg, text_color="red")
    elif level == "warning":
        log.print(final_msg, text_color="orange")
    elif level == "info":
        log.print(final_msg)


def log_error(msg, no_time=False):
    log(msg, "error", no_time)


def log_warning(msg, no_time=False):
    log(msg, "warning", no_time)


def log_info(msg, no_time=False):
    log(msg, "info", no_time)



###################################################################
#### UTILS FUNCTIONS ###########################################
#############################################################

def is_valid_file(filepath):
    if len(filepath) == 0:
        return False

    return os.path.isfile(filepath)

def is_valid_dir(dirpath):
    if len(dirpath) == 0:
        return False

    return os.path.isdir(dirpath)


def is_valid_number(num):
    return num.isdigit()


def update_start_button_state(values, is_thread_running):
    if      is_valid_dir(values[SCRIPT_FOLDER]) and \
            is_valid_file(values[BOOTLOADER]) and \
            is_valid_file(values[FIRMWARE]) and \
            is_valid_number(values[TIMEOUT_MOUNT]) and \
            len(values[MAINTENACE_MOUNT_NAME]) > 0 and \
            len(values[PROGRAM_MOUNT_NAME]) > 0 and \
            not is_thread_running:
        window[START_BUTTON].update(disabled=False)
    else:
        window[START_BUTTON].update(disabled=True)



###################################################################
#### OPENOCD FUNCTIONS #########################################
#############################################################

def openocd_procedure(values):
    log_info("------------------ START ------------------", True)
    start = time.time()
    is_ok = True

    steps(values)

    duration = int((time.time() - start) * 1000) 
    log_info("------------------ FINISH ------------------", True)
    log_info("Duration: {} s".format(duration / 1000.0), True)
    log_info("--------------------------------------------\n\n", True)


def steps(values):

    log_info("Unlocking the target (RDP)")
    if not openocd_unlock(values[SCRIPT_FOLDER]) :
        log_error("Failed to unlock the target... Abort")
        return

    log_info("Mass erase the target")
    if not openocd_mass_erase(values[SCRIPT_FOLDER]) :
        log_error("Failed to erase the target... Abort")
        return
        
    log_info("Flash the target")
    if not openocd_flash(values[BOOTLOADER], values[SCRIPT_FOLDER]) :
        log_error("Failed to flash the target... Abort")
        return
        
    log_info("Wait for device '{}' mount point".format(values[MAINTENACE_MOUNT_NAME]))
    if not openocd_wait_mountpoint(int(values[TIMEOUT_MOUNT]), values[MAINTENACE_MOUNT_NAME]) :
        log_error("Failed to open the target... Abort")
        return

    log_info("Search for 'Git SHA' from DETAILS.TXT in '{}' mount point: ".format(values[MAINTENACE_MOUNT_NAME]))
    openocd_read_SHA(values[MAINTENACE_MOUNT_NAME])

    log_info("Send firmware to device")
    if not openocd_copy_firmware(values[FIRMWARE], values[MAINTENACE_MOUNT_NAME]) :
        log_error("Failed to copy the firmware to the target... Abort")
        return

    log_info("Wait for device {} mount point".format(values[PROGRAM_MOUNT_NAME]))
    if not openocd_wait_mountpoint(int(values[TIMEOUT_MOUNT]), values[PROGRAM_MOUNT_NAME]) :
        log_error("Failed to open the target... Abort")
        return
    else:
        log_info("Search for 'Git SHA' from DETAILS.TXT of '{}' mount point: ".format(values[PROGRAM_MOUNT_NAME]))
        openocd_read_SHA(values[PROGRAM_MOUNT_NAME])

        if len(values[PROGRAM]) == 0:
            log_warning("Skipping programming steps")
            return
        else:
            log_info("Send program to device")
            if not openocd_copy_firmware(values[PROGRAM], values[PROGRAM_MOUNT_NAME]) :
                log_error("Failed to copy the program to the target... Abort")
                return


def openocd_unlock(script_folder):
    proc = subprocess.run(["openocd", "-s", script_folder, "-f", "configs/openocd-unlock.cfg"], capture_output=True, text=True)
    
    if proc.returncode == 0:
        return True

    log_info(proc.stdout, True)
    log_error(proc.stderr, True)
    return False


def openocd_mass_erase(script_folder):
    proc = subprocess.run(["openocd", "-s", script_folder, "-f", "configs/openocd-mass-erase.cfg"], capture_output=True, text=True)
    
    if proc.returncode == 0:
        return True

    log_info(proc.stdout, True)
    log_error(proc.stderr, True)
    return False


def openocd_flash(bootloader, script_folder):
    shutil.copy(bootloader, "./bootloader", follow_symlinks=True)
    proc = subprocess.run(["openocd", "-s", script_folder, "-f", "configs/openocd-flash.cfg"], capture_output=True, text=True)
    
    if proc.returncode == 0:
        return True

    log_info(proc.stdout, True)
    log_error(proc.stderr, True)
    return False


def openocd_wait_mountpoint(timeout, mount_point):
    start = time.time()
    tmp = start

    while True:

        partitions = psutil.disk_partitions()

        for p in partitions:
            if mount_point in p.mountpoint:
                return True

        if (time.time() - tmp) * 1000 > 1000:
            log_info("Waiting...")
            tmp = time.time()

        if (time.time() - start) * 1000 >= timeout:
            return False


def openocd_copy_firmware(file, mount_point):

    partitions = psutil.disk_partitions()
    target = None

    for p in partitions:
        if mount_point in p.mountpoint:
            target = p.mountpoint
            break

    if target == None:
        return False

    shutil.copy(file, target, follow_symlinks=True)

    return True

def openocd_read_SHA(mount_point):
    partitions = psutil.disk_partitions()
    path = None
    content = ""

    for p in partitions:
        if mount_point in p.mountpoint:
            path = p.mountpoint
            break

    if path == None :
        log_error("Failed to find the '{}' mountpoint".format(path))
        return

    with open("{}/DETAILS.TXT".format(path), "r") as f:
        content = f.read()
    
    res = re.search(r'Git SHA: ([a-zA-Z0-9]*)$', content, re.MULTILINE)

    if res != None:
        log_info(res.group(0))
    else:
        log_warning("No SHA found in file...")
    

window["-LOG-"].reroute_stdout_to_here()
window["-LOG-"].reroute_stderr_to_here()
main()
