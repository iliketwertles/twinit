extern crate sys_mount;

use sys_mount::{Mount, MountFlags};

pub fn early_mount() {
    println!("Mounting early filesystems...");
    let sysfs_result = Mount::builder()
        .fstype("sysfs")
        .mount("sysfs", "/sys");
    let devtmpfs_result = Mount::builder()
        .fstype("devtmpfs")
        .mount("tmpfs", "/dev");
    let proc_result = Mount::builder()
        .fstype("proc")
        .mount("proc", "/proc");
    let run_result = Mount::builder()
        .fstype("tmpfs")
        .mount("tmpfs", "run");
    
    match sysfs_result {
        Ok(_) => {println!("sysfs good :) ")},
        Err(e) => {println!("sysfs bad :( {}", e)}
    }
    match devtmpfs_result {
        Ok(_) => {println!("devtmpfs good :) ")},
        Err(e) => {println!("devtmpfs bad :( {}", e)}
    }
    match proc_result {
        Ok(_) => {println!("proc good :) ")},
        Err(e) => {println!("proc bad :( {}", e)}
    }
    match run_result {
        Ok(_) => {println!("run good :) ")},
        Err(e) => {println!("run bad :( {}", e)}
    }
    println!("Remounting rootfs as rw...");
    let remount_result = Mount::builder()
        .flags(MountFlags::REMOUNT)
        .mount("/dev/sda1", "/");
    // TODO: Change this to not be a static root device
    match remount_result {
        Ok(_) => {println!("Root is now rw!")}
        Err(e) => {println!("Root failed to remount: {}", e)}
    }
}