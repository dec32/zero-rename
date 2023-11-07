#![windows_subsystem = "windows"]
mod rename;
mod errors;
mod ui {
    pub(crate) mod console;
    pub(crate) mod win32;
}

fn main() {
    ui::win32::run();
    // ui::console::run();
}