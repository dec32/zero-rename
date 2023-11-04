mod rename;
mod errors;
mod ui {
    pub(crate) mod console;
    pub(crate) mod win32;
}

use std::path::Path;
use rename::Rename;

fn main() {
    // test();
    ui::console::run();
    // ui::win32::run();
}

#[allow(dead_code)]
fn test() {
    let path = Path::new("C:\\Users\\Administrator\\Documents\\test");
    let rename = Rename::preview(path).unwrap();
    rename.print();
}