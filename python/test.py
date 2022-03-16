import tsplib
import os

test_files = os.listdir("test_files")

for file in test_files:
    edges = tsplib.parse_file(f"test_files/{file}")
    print(f"File {file} done.")
