use glib::clone;
use gtk_macros::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use libadwaita::subclass::prelude::*;
use glib::WeakRef;
use gio::ApplicationFlags;


use once_cell::unsync::OnceCell;

use crate::config;
use crate::AdwaifehWindow;

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct AdwaifehApplication {
        pub window: OnceCell<WeakRef<AdwaifehWindow>>,
        pub settings: gio::Settings,
        pub current_directory: OnceCell<gio::File>,
        pub current_files: OnceCell<&'static [gio::File]>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AdwaifehApplication {
        const NAME: &'static str = "AdwaifehApplication";
        type Type = super::AdwaifehApplication;
        type ParentType = gtk::Application;

        fn new() -> Self {
            Self {
                window: OnceCell::new(),
                settings: gio::Settings::new("org.randev.Adwaifeh"),
                current_directory: OnceCell::new(),
                current_files: OnceCell::new(),

            }
        }

    }

    impl ObjectImpl for AdwaifehApplication {}


    impl ApplicationImpl for AdwaifehApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            let window = application.get_main_window();
            window.show();
            window.present();
        }


        // Entry point for GApplication
        fn startup(&self, application: &Self::Type) {
            self.parent_startup(application);

            let window = AdwaifehWindow::new(application);
            window.set_title(Some(&("Adwaifeh")));
            window.set_icon_name(Some(&config::APP_ID.to_owned()));
            self.window
                .set(window.downgrade())
                .expect("Failed to init application window");

            application.setup_actions();
            application.setup_accels();
        }


    }

    impl GtkApplicationImpl for AdwaifehApplication {}
    impl AdwApplicationImpl for AdwaifehApplication {}
}

glib::wrapper! {
    pub struct AdwaifehApplication(ObjectSubclass<imp::AdwaifehApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AdwaifehApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &config::APP_ID.to_owned()),
            ("flags", &ApplicationFlags::empty()),
            ("resource-base-path", &"/org/gnome/Adwaifeh".to_owned()),
        ])
        .unwrap()
    }

    pub fn get_main_window(&self) -> AdwaifehWindow {
        let imp = imp::AdwaifehApplication::from_instance(self);
        imp.window.get().unwrap().clone().upgrade().unwrap()
    }

    pub fn gsettings(&self) -> &gio::Settings {
        &imp::AdwaifehApplication::from_instance(self).settings
    }

    pub fn get_current_directory(&self) -> gio::File {
        let imp = imp::AdwaifehApplication::from_instance(self);
        imp.current_directory.get().unwrap().clone()
    }

    pub fn set_current_directory(&self, dir: Option<&gio::File>) {
        let imp = imp::AdwaifehApplication::from_instance(self);
        imp.current_directory.set(dir.unwrap().clone()).unwrap();
    }

    fn setup_actions(&self) {
        action!(
            self,
            "quit",
            clone!(@strong self as app => move |_,_| {
                app.quit();
            })
        );

        action!(
            self,
            "open-image",
            clone!(@strong self as app => move |_,_| {
                let win = app.get_main_window();

                //init file chooser dialog
                let dialog = gtk::FileChooserNative::new(
                    Some("Open Image"),
                    Some(&win),
                    gtk::FileChooserAction::Open,
                    Some("_Open"),
                    None,

                );

                //configure dialog
                dialog.set_modal(true);
                dialog.set_select_multiple(false);

                //add filters
                let filter = gtk::FileFilter::new();
                gtk::FileFilter::set_name(&filter, Some("Image files"));
                filter.add_mime_type("image/*");
                dialog.add_filter(&filter);

                let all_files_filter = gtk::FileFilter::new();
                gtk::FileFilter::set_name(&all_files_filter, Some("All files"));
                all_files_filter.add_pattern("*");
                dialog.add_filter(&all_files_filter);


                //TODO Add actual error handling here...
                dialog.connect_response(
                    clone!(@strong app, @strong dialog, @weak win as this => move |_, resp| {
                        if resp == gtk::ResponseType::Accept {
                            let file = dialog.file().unwrap();
                            this.update_image(&file);
                            app.set_current_directory(file.parent().as_ref());
                        }
                    }),
                );

                dialog.show();


            })
        );

        action!(
            self,
            "next-image",

        )
    }

    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("app.open-image", &["<primary>o"]);
    }


    fn show_about(&self) {

    }
}


impl Default for AdwaifehApplication {
    fn default() -> Self {
        gio::Application::default()
            .expect("Could not get default GApplication")
            .downcast()
            .unwrap()
    }
}

