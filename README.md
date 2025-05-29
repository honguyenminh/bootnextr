# bootnextr

A (hastily-written) Rust CLI to help manipulate BootNext NVRAM variable.

What is that jargon? Basically, if you dual-boot, and everytime you swap OS, you have to spam ESC or some other keys to open up the boot entry select menu, you will probably like this.

This selects the OS (or boot entry) that will be used ONLY on **the next time** your machine boots, without having to spam the key.

Be mindful I use full-text search because this is app from the 90s.

## Usage

Download the binary from the `Releases` tab on the right, or compile the project from source.

[Compile project](#compile)

Then run in your terminal for full help text:

```bash
bootnextr --help
```

Replace `bootnextr` with the path to the compiled CLI. You can put it anywhere, then add that folder to `PATH` for access from anywhere.

## Dependencies

This tool requires `efibootmgr` and `shutdown` on Linux, and `bcdedit` on Windows (which is pre-installed). Check if you can run these commands in your terminal.

Linux version can list boot entries without root permission, but all other functionalities, and the Windows version, requires elevated permission (root/Admin).

## Compile

Install rust toolchain (cargo, rustc) if you haven't done so.

Run `cargo build --release`, binary will be compiled into `target/release` folder.

**Happy multi-booting!**
