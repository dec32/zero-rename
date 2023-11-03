use std::{path::Path, io::stdin};

mod rename;
mod table;

fn main() {
    // test();
    run_in_console();
}

#[allow(dead_code)]
fn test() {
    let path = Path::new("C:\\Users\\Administrator\\Documents\\test");
    let mapping = rename::generate(path);
    table::print(&mapping, "Original", "New");
}

fn run_in_console() {
    loop {
        println!("Input the parent folder: ");
        let path = read_line();
        let path = Path::new(&path);
        let mapping = rename::generate(path);
        println!("Preview:");
        table::print(&mapping, "Original", "New");
        println!("Press Enter to confirm the renaming...");
        read_line();
    }
}

fn read_line() -> String{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}