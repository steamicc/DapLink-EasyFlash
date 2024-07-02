import time
import os
import sys
import threading
import subprocess
import shutil
import psutil
import re

import tkinter as tk
import tkinter.filedialog as tkFileDialog
import tkinter.messagebox as tkMessageBox
from tkinter import ttk

from settings import Settings
from logtext import LogText

SCRIPT_FOLDER = "-SCRIPT_FOLDER-"
BOOTLOADER = "-BOOTLOADER-"
FIRMWARE = "-FIRMWARE-"
PROGRAM = "-TEST_PROGRAM-"
TIMEOUT_MOUNT = "-TIMEOUT_MOUNT_POINT-"
MAINTENACE_MOUNT_NAME = "-MAINTENANCE_MOUNT_POINT-"
TARGET_MOUNT_NAME = "-PROGRAMMING_MOUNT_POINT-"


def ask_open_file(title_dialog, stringvar: tk.StringVar, setting_key: str | None):
    file_path = tkFileDialog.askopenfilename(filetypes=[("BIN files", "*.bin"), ("HEX files", "*.hex"), ("All Files", "*.* *")], title=title_dialog, initialdir=os.path.dirname(stringvar.get()))

    if file_path != () and file_path != "":
        stringvar.set(file_path)

        if setting_key != None:
            settings.set_value(setting_key, file_path)

def ask_open_folder(title_dialog, stringvar: tk.StringVar, setting_key: str | None):
    directory_path = tkFileDialog.askdirectory(mustexist=True, title=title_dialog, initialdir=stringvar.get())

    if directory_path != () and directory_path != "":
        stringvar.set(directory_path)

        if setting_key != None:
            settings.set_value(setting_key, directory_path)    

GRID_PADS = {'padx': '4px', 'pady': '2px'}
MAIN_PADS = {'padx': '4px', 'pady': '2px'}

settings = Settings("settings.dat")
window = tk.Tk(className='Easy Flash DapLink')
window.title("Easy Flash DapLink")


window.tk.call("source", "azure.tcl")
# window.tk.call("set_theme", "dark")
# ttk.Style().theme_use('azure-dark')
window.tk.call("set_theme", "light")
ttk.Style().theme_use('azure-light')

string_script_folder = tk.StringVar(value=settings.get_value_or_default(SCRIPT_FOLDER, "/usr/share/openocd/scripts/"))
string_file_bootloader = tk.StringVar(value=settings.get_value_or_default(BOOTLOADER, ""))
string_file_firmware = tk.StringVar(value=settings.get_value_or_default(FIRMWARE, ""))
string_file_test = tk.StringVar(value=settings.get_value_or_default(PROGRAM, ""))

string_maintenant_mount = tk.StringVar(value=settings.get_value_or_default(MAINTENACE_MOUNT_NAME, "MAINTENANCE"))
string_target_mount = tk.StringVar(value=settings.get_value_or_default(TARGET_MOUNT_NAME, "DIS_L4IOT" ))
string_timeout = tk.StringVar(value=settings.get_value_or_default(TIMEOUT_MOUNT, "10000" ))

log_text: LogText
running_thread: threading.Thread = None


def validate_number(num):
    return num.isdigit()

def main():
    global running_thread
    global log_text

    str_validate_number = (window.register(validate_number), '%P')

    ###
    # Panel files & folder 
    ###
    layout_files = ttk.Frame(master=window)
    layout_files.grid_columnconfigure(1, weight=1)
    
    ttk.Label(master=layout_files, text="OpenOCD 'scripts' folder").grid(column=0, row=0, sticky='NE', **GRID_PADS)
    ttk.Entry(master=layout_files, textvariable=string_script_folder, state=tk.NORMAL).grid(column=1, row=0, sticky='EW', **GRID_PADS)
    ttk.Button(master=layout_files, text="Browse...", command=lambda: ask_open_folder("Select the script directory", string_script_folder, SCRIPT_FOLDER)).grid(column=2, row=0, sticky='E', **GRID_PADS)

    ttk.Label(master=layout_files, text="Bootloader file").grid(column=0, row=1, sticky='E', **GRID_PADS)
    ttk.Entry(master=layout_files, textvariable=string_file_bootloader, state=tk.NORMAL).grid(column=1, row=1, sticky='EW', **GRID_PADS)
    ttk.Button(master=layout_files, text="Browse...", command=lambda: ask_open_file("Select the Bootloader file", string_file_bootloader, BOOTLOADER)).grid(column=2, row=1, sticky='E', **GRID_PADS)

    ttk.Label(master=layout_files, text="Firmware file").grid(column=0, row=2, sticky='E', **GRID_PADS)
    ttk.Entry(master=layout_files, textvariable=string_file_firmware, state=tk.NORMAL).grid(column=1, row=2, sticky='EW', **GRID_PADS)
    ttk.Button(master=layout_files, text="Browse...", command=lambda: ask_open_file("Select the firmware file", string_file_firmware, FIRMWARE)).grid(column=2, row=2, sticky='E', **GRID_PADS)

    ttk.Label(master=layout_files, text="Test file (skip if empty)").grid(column=0, row=3, sticky='E', **GRID_PADS)
    ttk.Entry(master=layout_files, textvariable=string_file_test, state=tk.NORMAL).grid(column=1, row=3, sticky='EW', **GRID_PADS)
    ttk.Button(master=layout_files, text="Browse...", command=lambda: ask_open_file("Select the test file", string_file_test, PROGRAM)).grid(column=2, row=3, sticky='E', **GRID_PADS)

    ###
    # Panel Mount points & timeout
    ###
    layout_params = ttk.Frame(master=window)
    layout_params.grid_columnconfigure(1, weight=1)
    
    ttk.Label(master=layout_params, text="'MAINTENANCE' mount point name").grid(column=0, row=0, sticky='E', **GRID_PADS)
    ttk.Entry(master=layout_params, textvariable=string_maintenant_mount).grid(column=1, row=0, sticky='EW', **GRID_PADS)

    ttk.Label(master=layout_params, text="Target mount point name").grid(column=0, row=1, sticky='E', **GRID_PADS)
    ttk.Entry(master=layout_params, textvariable=string_target_mount).grid(column=1, row=1, sticky='EW', **GRID_PADS)

    ttk.Label(master=layout_params, text="Timeout (in ms) for mount points").grid(column=0, row=2, sticky='E', **GRID_PADS)
    ttk.Entry(master=layout_params, textvariable=string_timeout, validate='all', validatecommand=str_validate_number).grid(column=1, row=2, sticky='EW', **GRID_PADS)

    ###
    # Panel Log
    ###
    layout_log = ttk.Frame(master=window)

    text_scroll_v = ttk.Scrollbar(master=layout_log, orient='vertical')
    text_scroll_v.pack(side=tk.RIGHT, fill=tk.Y)

    log_text = LogText(master=layout_log, yscrollcommand=text_scroll_v.set)

    text_scroll_v.config(command=log_text.yview)
    log_text.pack(fill=tk.BOTH, expand=True)

    ###
    # Pack the layout
    ###
    layout_files.pack(fill=tk.X, **MAIN_PADS)
    layout_params.pack(fill=tk.X, **MAIN_PADS)
    ttk.Button(master=window, style='Accent.TButton', text="Flash DAPLink", command=run_flash).pack(fill=tk.X, **MAIN_PADS)
    layout_log.pack(fill=tk.BOTH, expand=True, **MAIN_PADS)

    sys.stdout.write = redirector_stdout
    sys.stderr.write = redirector_stderr

    window.mainloop()
    
    settings.set_value(MAINTENACE_MOUNT_NAME, string_maintenant_mount.get())
    settings.set_value(TARGET_MOUNT_NAME, string_target_mount.get())
    settings.set_value(TIMEOUT_MOUNT, string_timeout.get())

def run_flash():
    global running_thread

    (check_ok, msg) = check_entries()

    if running_thread != None and running_thread.is_alive() == False:
        running_thread = None

    if running_thread != None:
        tkMessageBox.showwarning(title="Already running", message="Flash process is already running...", icon=tkMessageBox.WARNING)
        return
    elif not check_ok:
        tkMessageBox.showerror(title="Invalid entry", message=msg, icon=tkMessageBox.ERROR )
        return
    else:
        running_thread = threading.Thread(target=openocd_procedure, args=(string_script_folder.get(), string_file_bootloader.get(), string_file_firmware.get(), string_file_test.get(), string_maintenant_mount.get(), string_target_mount.get(), string_timeout.get()))
        running_thread.start()

###################################################################
#### UTILS FUNCTIONS ###########################################
#############################################################

def redirector_stdout(input_str):
    log_text.log_info(input_str, False, False)

def redirector_stderr(input_str):
    log_text.log_error(input_str, False, False)

def is_valid_file(filepath):
    if len(filepath) == 0:
        return False

    return os.path.isfile(filepath)

def is_valid_dir(dirpath):
    if len(dirpath) == 0:
        return False

    return os.path.isdir(dirpath)

def check_entries() -> tuple[bool, str]:
    if not is_valid_dir(string_script_folder.get()):
        return (False, "Invalid path to OpenOCD script folder")
    
    if not is_valid_file(string_file_bootloader.get()):
        return (False, "Invalid path to bootloader file")
    
    if not is_valid_file(string_file_firmware.get()):
        return (False, "Invalid path to firmware file")
    
    if len( string_file_test.get() ) > 0 and len( string_target_mount.get() ) > 0 and not is_valid_file(string_file_test.get()):
        return (False, "Invalid path to test file")
    
    if len( string_maintenant_mount.get() ) == 0:
        return (False, "Maintenace mount name can't be empty")
    
    if len( string_target_mount.get() ) == 0:
        return (False, "Target mount name can't be empty")

    return (True, "")

###################################################################
#### OPENOCD FUNCTIONS #########################################
#############################################################

def openocd_procedure(path_script: str, path_bootloader: str, path_firmware: str, path_program: str, mount_maintenance: str, mount_program: str, timeout: str):
    global running_thread

    log_text.log_info("------------------ START ------------------", True)
    start = time.time()
    is_ok = True

    steps(path_script, path_bootloader, path_firmware, path_program, mount_maintenance, mount_program, timeout)

    duration = int((time.time() - start) * 1000) 
    log_text.log_info("------------------ FINISH ------------------", True)
    log_text.log_info("Duration: {} s".format(duration / 1000.0), True)
    log_text.log_info("--------------------------------------------\n\n", True)
    running_thread = None

def steps(path_script: str, path_bootloader: str, path_firmware: str, path_program: str, mount_maintenance: str, mount_program: str, timeout: str):

    log_text.log_info("Unlocking the target (RDP)")
    if not openocd_unlock(path_script) :
        log_text.log_error("Failed to unlock the target... Abort")
        return

    log_text.log_info("Mass erase the target")
    if not openocd_mass_erase(path_script) :
        log_text.log_error("Failed to erase the target... Abort")
        return
        
    log_text.log_info("Flash the target")
    if not openocd_flash(path_bootloader, path_script) :
        log_text.log_error("Failed to flash the target... Abort")
        return
        
    log_text.log_info("Wait for device '{}' mount point".format(mount_maintenance))
    if not openocd_wait_mountpoint(int(timeout), mount_maintenance) :
        log_text.log_error("Failed to open the target... Abort")
        return

    log_text.log_info("Search for 'Git SHA' from DETAILS.TXT in '{}' mount point: ".format(mount_maintenance))
    openocd_read_SHA(mount_maintenance)

    log_text.log_info("Send firmware to device")
    if not openocd_copy_firmware(path_firmware, mount_maintenance) :
        log_text.log_error("Failed to copy the firmware to the target... Abort")
        return

    log_text.log_info("Wait for device {} mount point".format(mount_program))
    if not openocd_wait_mountpoint(int(timeout), mount_program) :
        log_text.log_error("Failed to open the target... Abort")
        return
    else:
        log_text.log_info("Search for 'Git SHA' from DETAILS.TXT of '{}' mount point: ".format(mount_program))
        openocd_read_SHA(mount_program)

        if len(path_program) == 0:
            log_text.log_warning("Skipping programming steps")
            return
        else:
            log_text.log_info("Send program to device")
            if not openocd_copy_firmware(path_program, mount_program) :
                log_text.log_error("Failed to copy the program to the target... Abort")
                return


def openocd_unlock(script_folder):
    proc = subprocess.run(["openocd", "-s", script_folder, "-f", "configs/openocd-unlock.cfg"], capture_output=True, text=True)
    
    if proc.returncode == 0:
        return True

    log_text.log_info(proc.stdout, False)
    log_text.log_error(proc.stderr, False)
    return False


def openocd_mass_erase(script_folder):
    proc = subprocess.run(["openocd", "-s", script_folder, "-f", "configs/openocd-mass-erase.cfg"], capture_output=True, text=True)
    
    if proc.returncode == 0:
        return True

    log_text.log_info(proc.stdout, False)
    log_text.log_error(proc.stderr, False)
    return False


def openocd_flash(bootloader, script_folder):
    shutil.copy(bootloader, "./bootloader", follow_symlinks=True)
    proc = subprocess.run(["openocd", "-s", script_folder, "-f", "configs/openocd-flash.cfg"], capture_output=True, text=True)
    
    if proc.returncode == 0:
        return True

    log_text.log_info(proc.stdout, False)
    log_text.log_error(proc.stderr, False)
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
            log_text.log_info("Waiting...")
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

    try:
        shutil.copy(file, target, follow_symlinks=True)
    except Exception as e:
        log_text.log_error("Failed to copy firmware file.\nError: {}".format(e))
        return False

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
        log_text.log_error("Failed to find the '{}' mountpoint".format(path))
        return

    try:
        with open("{}/DETAILS.TXT".format(path), "r") as f:
            content = f.read()
    except Exception as e:
        log_text.log_error("Unable to open read the Git SHA.\nError: {}".format(e))
        return

    res = re.search(r'Git SHA: ([a-zA-Z0-9]*)$', content, re.MULTILINE)

    if res != None:
        log_text.log_info(res.group(0))
    else:
        log_text.log_warning("No SHA found in file...")
    
main()
