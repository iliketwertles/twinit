extern crate ini;
use ini::Ini;
use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};

pub fn config() {
    // Check for config
    let conf_path = "/etc/twinit/twinit.ini";
    if Path::new(conf_path).exists() {
        println!("Config found, continuing...")
    } else {
        panic!("No config found, aborting!")
    }

    let conf = Ini::load_from_file(conf_path).unwrap();
    let section = conf.section(Some("Services_Path")).unwrap();
    let sv_path = section.get("path").unwrap();

    // Check for sv_path
    if Path::new(sv_path).exists() {
        // Start services
        for sv in fs::read_dir(sv_path).unwrap() {
            // Get bin location from service file
            println!("Starting process '{}'", sv.as_ref().unwrap().path().display());
            let service = Ini::load_from_file(sv.unwrap().path()).unwrap();
            let section = service.section(Some("Service")).unwrap();
            let exec = section.get("Start").unwrap();
            // Parse bin and args into variables (ugly tbh)
            let bin = exec.split_whitespace().next().unwrap();
            let exec_split = exec.split_whitespace();
            // Run bin with args, piping stdout and stderr to null 
            // TODO: Point stdout and stderr to log
            let mut run = Command::new(bin);
            run.args(exec_split).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();

        }
    }
}