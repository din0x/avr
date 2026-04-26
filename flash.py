import subprocess
import sys
from pathlib import Path

if len(sys.argv) < 2:
    print("Usage: flash.py <elf>")
    sys.exit(1)

elf_path = Path(sys.argv[1])
hex_path = elf_path.with_suffix(".hex")

subprocess.run([
    "../avr8-gnu/bin/avr-objcopy.exe",
    "-O", "ihex",
    "-R", ".eeprom",
    str(elf_path),
    str(hex_path)
], check=True)

subprocess.run([
    "../avrdude/avrdude.exe",
    "-c", "usbasp",
    "-p", "m16",
    "-U", f"flash:w:{hex_path}:i"
])
