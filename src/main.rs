mod conf;
mod tty;
mod udev;
mod early_mount;
pub use crate::conf::config;
use crate::tty::open_tty;
use crate::udev::udev;
use crate::early_mount::early_mount;

use std::thread;
fn main() {
    println!("Custom init started!");
    early_mount();
    udev();
    open_tty();
    config();
    thread::park();
}