# SnakeOS

SnakeOS is a small x86-64 operating system kernel written in Rust (`no_std`) and run in QEMU.

I started it to get a better understanding of what happens underneath the programs I normally write: interrupts, memory management, hardware input, and how those pieces fit together. It's still an experimental kernel, but it now has an interactive terminal and a basic filesystem running entirely in RAM.

## Features

- **Memory management:** Physical frame allocation from the bootloader memory map, virtual memory mapping with x86-64 paging, and a custom bump allocator managing a 64 KiB heap.
- **Interrupts and hardware:** GDT, IDT, TSS, basic exception handling, timer interrupts, PS/2 keyboard input, and serial logging.
- **Terminal:** Text rendering with an 8x8 bitmap font (scaled 3x), keyboard scancode decoding, command input, Enter, and Backspace.
- **Filesystem:** A small RAM filesystem supporting file creation, lookup, reading, writing, listing, and deletion. The current implementation supports 16 files, with up to 256 bytes of contents and 16-byte filenames per file.

The terminal currently includes commands such as `help`, `say`, `create`, `write`, `read`, `ls`, and `delete`.

## Demo



https://github.com/user-attachments/assets/21951440-54ee-4071-aefb-adb3bd7c1881



## Build and run

You'll need a Rust nightly toolchain and QEMU (`qemu-system-x86_64`) installed. From the repository root:

```sh
cargo run
```

The build script creates a BIOS boot image, and the launcher starts QEMU with serial output connected to the host terminal.

## Limitations

- The filesystem is in memory; files disappear after a reboot. There are no directories or persistent disk storage yet.
- The heap uses a bump allocator, so freed allocations are not reclaimed.
- Terminal input and rendering are basic, and there is no userspace or process scheduling.

The goal is to keep learning about systems programming by building and improving the kernel one subsystem at a time.
