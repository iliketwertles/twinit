use std::process::{Command, Stdio};
use std::fs;
use system_shutdown::reboot;


pub fn ls(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

pub fn open_tty() {
    println!("Populating /dev (hopefully)...");
    let mount = Command::new("mount")
        .arg("-nt")
        .arg("devtmpfs")
        .arg("none")
        .arg("/dev")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match mount {
        Ok(_) => {
            println!("Mount thinks it worked");
            ls("/dev");
        },
        Err(e) => {
            println!("Mount thinks it didn't work... {}", e);
            println!("You wanna reboot?");
            let mut input = String::new();
            let _b1 = std::io::stdin().read_line(&mut input).unwrap();
            if input == "y" {
                match reboot() {
                    Ok(_) => {println!("Rebooting...")},
                    Err(e) => {println!("Can't reboot... panicing ! {}", e)},
                }
            }
        },
    }
    println!("remounting rootfs as rw...");
    let mount2 = Command::new("mount")
        .arg("-o")
        .arg("remount,rw")
        .arg("/")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match mount2 {
        Ok(_) => {
            println!("Mount2 thinks it worked");
            ls("/dev");
        },
        Err(e) => {
            println!("Mount2 thinks it didn't work... {}", e);
            println!("You wanna reboot?");
            let mut input = String::new();
            let _b1 = std::io::stdin().read_line(&mut input).unwrap();
            if input == "y" {
                match reboot() {
                    Ok(_) => {println!("Rebooting...")},
                    Err(e) => {println!("Can't reboot... panicing ! {}", e)},
                }
            }
        },
    }


    let mknod = Command::new("mknod")
        .args(["-m", "666", "/dev/tty1", "c", "5", "0"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match mknod {
        Ok(_) => {println!("mknod thinks it worked")},
        Err(e) => {
            println!("mknod thinks it didn't work... {}", e);
            println!("You wanna reboot?");
            let mut input = String::new();
            let _b1 = std::io::stdin().read_line(&mut input).unwrap();
            if input == "y" {
                match reboot() {
                    Ok(_) => {println!("Rebooting...")},
                    Err(e) => {println!("Can't reboot... panicing ! {}", e)},
                }
            }
        },
    }

    println!("Starting tty...");
    let agetty = Command::new("/sbin/agetty")
        //.arg("-h")
        .arg("9600")
        .arg("tty1")
        .arg("vtxxx")
        .spawn();
    match agetty {
        Ok(_) => {println!("Agetty thinks it worked")},
        Err(e) => {
            println!("Agetty thinks it didn't work... {}", e);
            println!("You wanna reboot?");
            let mut input = String::new();
            let _b1 = std::io::stdin().read_line(&mut input).unwrap();
            if input == "y".to_string() {
                match reboot() {
                    Ok(_) => {println!("Rebooting...")},
                    Err(e) => {println!("Can't reboot... panicing ! {}", e)},
                }
            }
        },
    }   
}
