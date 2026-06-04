import os
import argparse

def read_airy_file():
    parser = argparse.ArgumentParser()
    parser.add_argument("path_file", type=str)
    args = parser.parse_args()

    if not os.path.exists(args.path_file):
        raise FileNotFoundError("File does not exist")

    if not args.path_file.endswith(".airy"):
        raise ValueError("Only .airy files allowed")

    with open(args.path_file, "r", encoding="utf-8") as f:
        return f.read()