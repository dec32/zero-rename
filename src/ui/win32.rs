use std::{rc::Rc, path::Path, collections::HashMap, cell::Cell};

use native_windows_gui as nwg;
use nwg::{Window, Button, Event, FileDialog, FileDialogAction, ListView, InsertListViewColumn, ListViewStyle, InsertListViewItem, FlexboxLayout, stretch::{style::{FlexDirection, Dimension}, geometry::Size}, Font};

use crate::{rename::Rename, errors::Error};



pub fn run(path :Option<&Path>) {
    // I hate rust
    let mut rename = Cell::new(None);
    if let Some(path) = path {
        match Rename::preview(path) {
            Ok(value) => rename.set(Some(value)),
            Err(error) => handle_error(error)
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
    if let Some(rename) = rename.get_mut() {
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
    FlexboxLayout::builder()
        .parent(&window)
        .flex_direction(FlexDirection::Column)
        .child(&preview)
            .child_size(Size { width: Dimension::Auto, height: Dimension::Points(400.0)})
        .child(&dir_chooser_btn)
            .child_size(Size { width: Dimension::Auto, height: Dimension::Points(40.0)})
        .child(&confirm_btn)
            .child_size(Size { width: Dimension::Auto, height: Dimension::Points(40.0)})
        .build(&mut main_layout).unwrap();


    // styling
    Font::set_global_family("Segoe UI").unwrap();



    // what't the point of rc and clone tho?
    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |event, _, handle| {
        match event {
            Event::OnWindowClose => 
                if &handle == &events_window as &Window {
                    // to kill the whole process
                    nwg::stop_thread_dispatch();
                },
    
            Event::OnButtonClick => {    
                if &handle == &dir_chooser_btn.handle {
                    println!("Choose Folder");
                    dir_chooser.run(Some(events_window.handle));
                    let Ok(path) = dir_chooser.get_selected_item() else {
                        return;
                    };
                    let path = Path::new(path.to_str().unwrap());
                    match Rename::preview(path) {
                        Ok(ok) =>  {
                            update_preview(&preview, ok.mapping());
                            rename.set(Some(ok));
                        }
                        Err(error) => {    
                            handle_error(error);
                            return;
                        }
                    }
                    
                } else if &handle == &confirm_btn.handle {
                    println!("Confirm");
                    let rename = rename.take();
                    let Some(rename) = rename else {
                        println!("No folder");
                        return;
                    };
                    if let Err(err) = rename.apply() {
                        handle_error(err);
                    }
                }
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

fn update_preview(preview: &ListView, mapping: &HashMap<String, String>) {
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