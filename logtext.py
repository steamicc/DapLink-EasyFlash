from enum import Enum
from datetime import datetime

import tkinter as tk
from tkinter import ttk

class LogType(Enum):
    INFO = "INFO",
    WARNING = "WARNING",
    ERROR = "ERROR"


class LogText(tk.Text):
    def __init__(self, **kw):
        super().__init__(**kw)
        self.tag_configure(LogType.INFO, foreground="#000000")
        self.tag_configure(LogType.WARNING, foreground="#FFB54C")
        self.tag_configure(LogType.ERROR, foreground="#FF6961")

    def log(self, type: LogType, text: str, print_time: bool = True):
        final_msg = ""

        self.configure(state=tk.NORMAL)

        if print_time:
            final_msg = "[{}]    {}".format(datetime.now().strftime("%H:%M:%S.%f"), text)
        else:
            final_msg = text

        super().insert(tk.END, final_msg, type)
        super().see(tk.END)
        self.configure(state=tk.DISABLED)

    def log_error(self, text: str, print_time: bool = True, add_newline: bool = True):
        if add_newline :
            text += "\n"

        self.log(LogType.ERROR, text, print_time)

    def log_warning(self, text: str, print_time: bool = True, add_newline: bool = True):
        if add_newline :
            text += "\n"

        self.log(LogType.WARNING, text, print_time)

    def log_info(self, text: str, print_time: bool = True, add_newline: bool = True):
        if add_newline :
            text += "\n"

        self.log(LogType.INFO, text, print_time)