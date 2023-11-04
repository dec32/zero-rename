use std::{rc::Rc, path::Path, collections::HashMap, error};

use native_windows_gui as nwg;
use nwg::{Window, Button, Event, FileDialog, FileDialogAction, ListView, InsertListViewColumn, ListViewStyle, InsertListViewItem, GridLayout};

use crate::{rename::Rename, errors::Error};


pub fn run(path :Option<&Path>) {
    let mut rename = None;
    if let Some(path) = path {
        match Rename::preview(path){
            Ok(value) => rename = Some(value),
            Err(error) => handle_error(error),
        }
    }

    nwg::init().unwrap();
    // main window
    let mut window = Default::default();
    Window::builder()
        .title("ZeroRename")
        .build(&mut window).unwrap();


    // preview area
    let mut preview = Default::default();
    ListView::builder()
        .parent(&window)
        .list_style(ListViewStyle::Detailed)
        .build(&mut preview)
        .unwrap();

    // preview header
    preview.set_headers_enabled(true);
    preview.insert_column(InsertListViewColumn {
        index: Some(0),
        fmt: None,
        width: None,
        text: Some(String::from("Original")),
    });

    preview.insert_column(InsertListViewColumn {
        index: Some(1),
        fmt: None,
        width: None,
        text: Some(String::from("Renamed to")),
    });

    // preview items
    if let Some(rename) = rename {
        update_preview(&mut preview, rename.mapping());
    }

    // dir chooser btn
    let mut dir_chooser_btn = Default::default();
    Button::builder()
        .parent(&window)
        .text("Choose Folder")
        .build(&mut dir_chooser_btn).unwrap();


    // dir chooser
    let mut dir_chooser = Default::default();
    FileDialog::builder()
        .action(FileDialogAction::OpenDirectory)
        .multiselect(false)
        .build(&mut dir_chooser).unwrap();


    // confirm button
    let mut confirm_btn = Default::default();
    Button::builder()
        .parent(&window)
        .text("Confirm")
        .build(&mut confirm_btn).unwrap();


    // layout
    let mut main_layout = Default::default();
    GridLayout::builder()
        .parent(&window)
        .child(0, 0, &preview)
        .child(0, 1, &dir_chooser_btn)
        .child(0, 2, &confirm_btn)
        .build(&mut main_layout).unwrap();



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
                // FIXME some closure issue. variable cannot be captured

                // if &handle == &dir_chooser_btn.handle {
                //     println!("Choose Folder");
                //     dir_chooser.run(Some(events_window.handle));
                //     let path = dir_chooser.get_selected_item().unwrap();
                //     let path = path.to_str().unwrap();
                //     let path = Path::new(path);
                //     match Rename::preview(path) {
                //         Ok(value) => rename = Some(value),
                //         Err(error) => {    
                //             handle_error(error);
                //             return;
                //         }
                //     }
                //     // update_preview(&mut preview, rename.mapping())
                // } else if &handle == &confirm_btn.handle {
                //     println!("Confirm");
                //     let Some(rename) = rename else {
                //         println!("No folder");
                //         return;
                //     };
                //     if let Err(err) = rename.apply() {
                //         handle_error(err);
                //     }
                // }
            },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}


fn old_name_item(index: i32, text: String) -> InsertListViewItem{
    InsertListViewItem {
        index:Some(index),
        column_index: 0,
        text: Some(text),
        image:None
    }
}

fn new_name_item(index: i32, text: String) -> InsertListViewItem{
    InsertListViewItem {
        index:Some(index),
        column_index: 1,
        text: Some(text),
        image:None
    }
}

fn update_preview(preview: &mut ListView, mapping: &HashMap<String, String>) {
    preview.clear();
    let mut index = 0;
    for (old_name, new_name) in mapping {
        preview.insert_item(old_name_item(index, old_name.to_owned()));
        preview.insert_item(new_name_item(index, new_name.to_owned()));
        index += 1;
    }
}

fn handle_error(error: Error) {
    println!("{}", error)
}