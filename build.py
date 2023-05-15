#!/usr/bin/python3
import os
from shutil import copyfile as cp
import stat


dir = os.getcwd()
bin_dir = dir+"/bin"

try:
    os.mkdir("bin")
except:
    pass

os.chdir(dir+"/src/rust/")
for i in os.listdir():
    os.chdir(dir+"/src/rust/"+i)
    os.system("cargo build --release")
    os.chdir("./target/release")
    files = []
    for f in os.listdir():
        if os.path.isfile(f):
            if f.endswith(".d") or (f == ".cargo-lock"):
                continue
            files.append(f)
    if len(files) == 1:
        binary = files[0]

    elif len(files) == 0:
        print("BINARY FILE NOT FOUND!!!")

    else:
        binary = ""
        input_valid = False
        while not input_valid:
            print(f"possible files are: ")
            for idx, f in enumerate(files):
                print(f"{idx+1}: {f}")
            try:
                inp = int(input("What file is correct?: "))-1
                binary = files[inp]
                input_valid = True
            except:
                print("File not valid")
    cp(binary, f"{bin_dir}/{binary}")
    st = os.stat(f"{bin_dir}/{binary}")
    os.chmod(f"{bin_dir}/{binary}", st.st_mode | stat.S_IEXEC)