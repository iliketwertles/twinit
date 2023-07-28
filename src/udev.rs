use std::process::{Command, Stdio};

pub fn udev() {
    println!("Populating /dev...");
    let udevd = Command::new("/sbin/udevd")
        .arg("--daemon")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match udevd {
        Ok(_) => {println!("Populated /dev successfully")},
        Err(e) => {println!("Populating /dev failed: {}", e)},
    }

    println!("Letting devices settle...");
    let settle = Command::new("/sbin/udevadm")
        .arg("settle")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match settle {
        Ok(_) => {println!("Devices have settled")},
        Err(e) => {println!("Devices can not settle (time to panic): {}", e)},
    }
}