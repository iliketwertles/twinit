use std::process::{Command, Stdio};
use system_shutdown::reboot;


pub fn open_tty() {
    let mknod = Command::new("mknod")
        .args(["-m", "666", "/dev/tty1", "c", "4", "1"])
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
        .arg("-h")
        .arg("'-p -- \\u'")
        .arg("--noclear")
        .arg("-")
        .arg("linux")
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