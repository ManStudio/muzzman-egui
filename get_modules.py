#!/usr/bin/python

import os
import shutil

# This only should me used only if you using the muzzman-workspace

if not os.path.isdir("modules"):
    os.mkdir("modules")

for file in os.listdir("modules"):
    os.remove(f"modules/{file}")

for dir in os.listdir("../modules"):
    dir = f"../modules/{dir}"
    if os.path.isdir(dir):
        for file in os.listdir(dir):
            if ".so" in file:
                shutil.copyfile(f"{dir}/{file}", f"modules/{file}")
