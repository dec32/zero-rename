use std::{path::Path, io::stdin, env};

use crate::rename::Rename;

#[allow(dead_code)]
pub fn run() {
    match env::args().nth(1) {
        None => interact(),
        Some(arg) => match arg.as_str() {
            "-h" | "--help" => print_help(),
            _ => rename_paths()
        }        
    }
}

fn interact() {
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
        print!("{}", rename);
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


fn rename_paths() {
    let args: Vec<String> = env::args().collect();
    let mut paths = Vec::with_capacity(args.len());
    for arg in args.iter() {
        let path = Path::new(arg);
        if path.is_file() {
            println!("Path {} is no a directory.", path.display())
        } else {
            paths.push(path);
        }
    }
    if paths.len() != args.len() {
        return;
    }
    for path in paths {
        Rename::rename(path).unwrap();
    }
}

fn print_help() {
    println!("Use no argument to run the interactive CLI.");
    println!("Use '-h' or '--help' to print help.");
    println!("Use paths as arguments to rename the files in them.");
}
