mod conf;
mod tty;
pub use crate::conf::config;
use crate::tty::open_tty;
fn main() {
    println!("Custom init started!");
    open_tty();
    config();
    loop {}
}