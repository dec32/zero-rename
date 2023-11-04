use std::{path::Path, io::stdin};
use rename::Rename;
mod rename;
mod errors;

fn main() {
    // test();
    run_in_console();
}

#[allow(dead_code)]
fn test() {
    let path = Path::new("C:\\Users\\Administrator\\Documents\\test");
    let rename = Rename::preview(path).unwrap();
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
        if let Err(err) = rename {
            println!("Error occured while accessing the directory.");
            println!("{}", err);
            continue;
        }
        let rename = rename.unwrap();
        if rename.is_empty() {
            println!("No file needs to be renamed.");
            continue;
        }
        println!("Preview:");
        rename.print();
        println!("Press Enter to confirm the renaming...");
        read_line();
        let res = rename.apply();
        if let Err(err) = res {
            println!("Error occured while renaming the files.");
            println!("{}", err);
            continue;
        }
        println!("Renamed files in [{}] successfully.", path.display());
    }
}


fn read_line() -> String{
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}