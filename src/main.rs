// SoonTM
// #[cfg(target_os = "windows")]
// extern crate native_windows_gui as nwg;
// extern crate native_windows_derive as nwd;

use sysinfo::{System, SystemExt};

fn get_sysinfo() {
    println!("--- SYSINFO ---");
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("Hostname: {}", sys.host_name().unwrap());
}

#[cfg(target_os = "windows")]
fn main() {
    println!("Hello, Windows!");
    get_sysinfo();
}

#[cfg(target_os = "macos")]
fn main() {
    println!("Hello, macOS!");
    get_sysinfo();
}

#[cfg(target_os = "linux")]
fn main() {
    println!("Hello, Linux!");
    get_sysinfo();
}