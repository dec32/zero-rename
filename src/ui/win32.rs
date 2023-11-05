use std::{rc::Rc, path::Path, cell::Cell, ops::Deref};
use native_windows_gui as nwg;
use nwg::{Window, Button, Event, FileDialog, FileDialogAction, ListView, InsertListViewColumn, ListViewStyle, FlexboxLayout, stretch::{style::{FlexDirection, Dimension}, geometry::Size}, Font, InsertListViewItem};
use crate::{rename::Rename, errors::Error};


pub fn run() {
    nwg::init().unwrap();
    let app = App::new();
    let wrapper = AppWrapper::new(app);
    wrapper.run();
}

pub fn run_under(path:&Path) {
    nwg::init().unwrap();
    let app = App::new();
    app.switch_dir(path);
    let wrapper = AppWrapper::new(app);
    wrapper.run();
}


struct App {
    // view
    window: Window,
    preview: ListView,
    dir_chooser: FileDialog,
    dir_chooser_btn: Button,
    confirm_btn: Button,
    // model
    rename: Cell<Option<Rename>>,
}


impl App {
    fn new() -> Self {
        let mut app = App {
            window: Default::default(),
            preview: Default::default(),
            dir_chooser: Default::default(),
            dir_chooser_btn: Default::default(),
            confirm_btn: Default::default(),
            rename: Cell::new(None)
        };
        app.build_ui();
        app
    }

    fn choose_dir(&self) {
        self.dir_chooser.run(Some(self.window.handle));
        let Ok(path) = self.dir_chooser.get_selected_item() else {
            return;
        };
        let path = Path::new(&path);
        self.switch_dir(path);
    }

    pub fn switch_dir(&self, path: &Path) {
        let parent = Path::new(path);
        match Rename::preview(parent) {
            Ok(rename) =>  {
                self.update(rename);
            }
            Err(error) => {
                self.clear();
                alert_error("Failed to infer the files to be renamed.", error);
                return;
            }
        }
    }

    fn confirm(&self) {
        let rename = self.rename.take();
        let Some(rename) = rename else {
            alert_msg("Please choose the folder cotaining the files you want to rename.");
            self.rename.set(rename);
            return;
        };
        match rename.apply() {
            Ok(_) => {
                if rename.is_empty() {
                    alert_msg("No file needs to be renamed")
                } else {
                    alert_msg("Renamed the files successfully.");
                }
                self.clear();
            },
            Err(error) => {
                self.rename.set(Some(rename));
                alert_error("Failed to rename the files.", error);
                self.clear();
            }
        }
    }

    fn clear(&self) {
        self.preview.clear();
        self.window.set_text("ZeroRename");
        self.rename.set(None);
    }

    fn update(&self, rename: Rename) {
        self.preview.clear();
        let mut index = 0;
        for (old_name, new_name) in rename.mapping() {
            self.preview.insert_item(old_name_item(index, old_name.to_owned()));
            self.preview.insert_item(new_name_item(index, new_name.to_owned()));
            index += 1;
        }
        let path = rename.parent().to_str().unwrap_or("");
        self.window.set_text(format!("ZeroRename[{}]", path).as_str());
        self.rename.set(Some(rename));
    }

    fn build_ui(&mut self) {
        // main window
        self.window = Default::default();
        Window::builder()
            .title("ZeroRename")
            .build(&mut self.window).unwrap();
    
        // preview area
        self.preview = Default::default();
        ListView::builder()
            .parent(&self.window)
            .list_style(ListViewStyle::Detailed)
            .build(&mut self.preview)
            .unwrap();
    
        // preview header
        self.preview.set_headers_enabled(true);
        self.preview.insert_column(InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: None,
            text: Some(String::from("Original")),
        });
    
        self.preview.insert_column(InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: None,
            text: Some(String::from("Renamed to")),
        });
    
        // dir chooser btn
        self.dir_chooser_btn = Default::default();
        Button::builder()
            .parent(&self.window)
            .text("Choose Folder")
            .build(&mut self.dir_chooser_btn).unwrap();
    
    
        // dir chooser
        self.dir_chooser = Default::default();
        FileDialog::builder()
            .action(FileDialogAction::OpenDirectory)
            .multiselect(false)
            .build(&mut self.dir_chooser).unwrap();
    
    
        // confirm button
        self.confirm_btn = Default::default();
        Button::builder()
            .parent(&self.window)
            .text("Confirm")
            .build(&mut self.confirm_btn).unwrap();
    
    
        // layout
        let mut main_layout = Default::default();
        FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)
            .child(&self.preview)
                .child_size(Size { width: Dimension::Auto, height: Dimension::Points(400.0)})
            .child(&self.dir_chooser_btn)
                .child_size(Size { width: Dimension::Auto, height: Dimension::Points(40.0)})
            .child(&self.confirm_btn)
                .child_size(Size { width: Dimension::Auto, height: Dimension::Points(40.0)})
            .build(&mut main_layout).unwrap();
    
    
        // styling
        Font::set_global_family("Segoe UI").unwrap();

    }
}


struct AppWrapper {
    app: Rc<App>
}

impl AppWrapper {
    fn new(app:App) -> Self{
        AppWrapper{app:Rc::new(app)}
    }

    fn run (&self) {
        let app = Rc::downgrade(&self.app);
        let handler = nwg::full_bind_event_handler(&self.app.window.handle, move |event, _, handle| {
            let Some(app) = app.upgrade() else {
                return;
            };
            match event {
                Event::OnWindowClose => {
                    if &handle == &app.window.handle {nwg::stop_thread_dispatch()}
                }
                Event::OnButtonClick => {
                    if &handle == &app.dir_chooser_btn.handle {app.choose_dir()}
                    else if &handle == &app.confirm_btn.handle {app.confirm()}
                }
                _ => {}
            }
        });
        nwg::dispatch_thread_events();
        nwg::unbind_event_handler(&handler);
    }
}

impl Deref for AppWrapper {
    type Target = App;
    fn deref(&self) -> &App {
        &self.app
    }
}


fn alert_msg(msg: &str) {
    nwg::simple_message("Message", msg);
}

fn alert_error(msg: &str, error: Error) {
    let msg = format!("{}\n{}", msg, error);
    nwg::error_message("Error", &msg);
    println!("{}", error)
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