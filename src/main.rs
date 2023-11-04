use std::{path::Path, io::stdin};
use rename::Rename;
mod rename;

fn main() {
    // test();
    run_in_console();
}

#[allow(dead_code)]
fn test() {
    let path = Path::new("C:\\Users\\Administrator\\Documents\\test");
    let rename = Rename::preview(path);
    rename.print();
}

fn run_in_console() {
    loop {
        println!("Input the parent folder(or left empty to quit the program): ");
        let path = read_line();
        if path.len() == 0 {
            break;
        }
        let path = Path::new(&path);
        let rename = Rename::preview(path);
        if rename.is_empty() {
            println!("No file needs to be renamed.");
            continue;
        }
        println!("Preview:");
        rename.print();
        println!("Press Enter to confirm the renaming...");
        read_line();
        rename.apply();
        println!("Renamed files in [{}] successfully.", path.display());
    }
}

fn read_line() -> String{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}