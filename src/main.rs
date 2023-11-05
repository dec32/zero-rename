#![windows_subsystem = "windows"]
mod rename;
mod errors;
mod ui {
    pub(crate) mod console;
    pub(crate) mod win32;
}

use std::{path::Path, env};
use rename::Rename;
fn main() {
    match env::args().nth(1) {
        None => ui::win32::run(),
        Some(arg) => match arg.as_str() {
            "-i" | "--interact" => ui::console::run(),
            "-h" | "--help" => print_help(),
            _ => rename_path(arg)
        }        
    }
}

fn rename_path(path: String) {
    Rename::rename(Path::new(&path)).unwrap();
}

fn print_help() {
    println!("Use no argument to run the GUI.");
    println!("Use '-h' or '--help' to print help.");
    println!("Use '-i' or '--interact' to run the interactive CLI.");
    println!("Use path as argument to rename the files in it.");
}

#[allow(dead_code)]
fn test() {
    let path = Path::new("C:\\Users\\Administrator\\Documents\\test");
    let rename = Rename::preview(path).unwrap();
    print!("{}", rename)
}