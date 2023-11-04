use std::rc::Rc;

use native_windows_gui as nwg;
use nwg::{Window, Button, Event, FileDialog, FileDialogAction};

pub fn run() {
    nwg::init().unwrap();
    let mut window = Default::default();
    let mut dir_chooser_btn = Default::default();
    let mut dir_chooser = Default::default();

    Window::builder()
        .title("ZeroRename")
        .build(&mut window).unwrap();

    Button::builder()
        .text("Choose Folder")
        .parent(&window)
        .build(&mut dir_chooser_btn)
        .unwrap();

    FileDialog::builder()
        .action(FileDialogAction::OpenDirectory)
        .build(&mut dir_chooser)
        .unwrap();


    // what't the point of rc and clone tho?
    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |event, event_data, handle| {
        match event {
            Event::OnWindowClose => 
                if &handle == &events_window as &Window {
                    // to kill the whole process
                    nwg::stop_thread_dispatch();
                },
            Event::OnButtonClick => {
                if &handle == &dir_chooser_btn.handle {
                    dir_chooser.run(Some(events_window.handle));
                }
            },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}