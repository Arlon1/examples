//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;

    use gtk::prelude::*;
    use gtk::{self, Builder};

    pub fn sub_main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let glade_src = include_str!("text_viewer.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: gtk::Window = builder.get_object("window").unwrap();
        let open_button: gtk::ToolButton = builder.get_object("open_button").unwrap();
        let text_view: gtk::TextView = builder.get_object("text_view").unwrap();

        let window1 = window.clone();
        open_button.connect_clicked(move |_| {
            // TODO move this to a impl?
            let file_chooser = gtk::FileChooserDialog::new(
                Some("Open File"), Some(&window1), gtk::FileChooserAction::Open);
            file_chooser.add_buttons(&[
                ("Open", gtk::ResponseType::Ok.into()),
                ("Cancel", gtk::ResponseType::Cancel.into()),
            ]);
            if file_chooser.run() == gtk::ResponseType::Ok.into() {
                let filename = file_chooser.get_filename().unwrap();
                let file = File::open(&filename).unwrap();

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view.get_buffer().unwrap().set_text(&contents);
            }

            file_chooser.destroy();
        });

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();
        gtk::main();
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::sub_main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example only work with GTK 3.10 and later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}
