use std::process::Command;

fn main() {
    let bios_path = env!("BIOS_PATH");

    Command::new("qemu-system-x86_64")
        .args(["-drive", &format!("format=raw,file={bios_path}")])
        .args(["-serial", "stdio"])
        .status()
        .expect("failed to start QEMU");
}
