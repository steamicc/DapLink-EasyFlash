from json import load
import pickle
import os
from sys import stderr

class Settings:

    def __init__(self, filepath) -> None:
        self.dict = {}
        self.filepath = filepath
        self.load()

    def set_value(self, key, value):
        self.dict[key] = value
        self.save()

    def get_value(self, key):
        return self.get_value_or_default(key, None)

    def get_value_or_default(self, key, default):
        if key in self.dict :
            return self.dict[key]
        else:
            return default

    def save(self):
        try:
            with open(self.filepath, "wb") as f:
                pickle.dump(self.dict, f)

        except Exception as err:
            print(err, file=stderr)

    def load(self):
        if not os.path.isfile(self.filepath) :
            print("No file to load for settings")
            return
        
        try:
            with open(self.filepath, "rb") as f:
                self.dict = pickle.load(f)

        except Exception as err:
            print(err, file=stderr)
