#!/usr/bin/env python3

import subprocess
from pathlib import Path
from rich import print

# subprocess.run(
#     ['cargo', 'clean'],
#     encoding='utf-8'
# ).check_returncode()

bin = 'literal'

subprocess.run(
    ['cargo', 'build', '--release', '--bin', f'{bin}'],
    encoding='utf-8'
).check_returncode()

subprocess.run(
    ['hyperfine', f"cargo run --bin {bin}"],
    encoding="utf-8"
).check_returncode()

builded_file = Path(f"./target/release/{bin}")

print(f"builded file size is [red]{builded_file.stat().st_size/1024} KiB.")
