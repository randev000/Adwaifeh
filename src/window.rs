use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, gdk, CompositeTemplate};
use libadwaita::subclass::prelude::*;
use glib::{clone};
use std::time::Duration;
use crate::AdwaifehApplication;
use std::cell::Cell;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/randev/Adwaifeh/window.ui")]
    pub struct AdwaifehWindow {

        pub menu_bool: Cell<bool>,

        // Template widgets
        #[template_child]
        pub main_image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub click_handler: TemplateChild<gtk::GestureClick>,
        #[template_child]
        pub window_flap: TemplateChild<libadwaita::Flap>,
        #[template_child]
        pub window_title: TemplateChild<libadwaita::WindowTitle>,
        #[template_child]
        pub headerbar: TemplateChild<gtk::HeaderBar>,


    }

    #[glib::object_subclass]
    impl ObjectSubclass for AdwaifehWindow {
        const NAME: &'static str = "AdwaifehWindow";
        type Type = super::AdwaifehWindow;
        type ParentType = libadwaita::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AdwaifehWindow {}
    impl WidgetImpl for AdwaifehWindow {}
    impl WindowImpl for AdwaifehWindow {}
    impl ApplicationWindowImpl for AdwaifehWindow {}
    impl AdwApplicationWindowImpl for AdwaifehWindow {}
}

glib::wrapper! {
    pub struct AdwaifehWindow(ObjectSubclass<imp::AdwaifehWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,libadwaita::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl AdwaifehWindow {
    pub fn new<P: IsA<gtk::Application> + glib::value::ToValue>(application: &P) -> Self {
       let win :AdwaifehWindow= glib::Object::new(&[("application", application)])
            .expect("Failed to create AdwaifehWindow");

        win.init();

        win
    }

    fn imp(&self) -> &imp::AdwaifehWindow {
        imp::AdwaifehWindow::from_instance(self)
    }




    fn init(&self) {
        let imp = self.imp();

        let style : libadwaita::StyleManager = libadwaita::StyleManager::default();
        style.set_color_scheme(libadwaita::ColorScheme::ForceDark);


        imp.click_handler.connect_released(clone!(@weak self as win => move |_, _, _,_| {
            win.on_win_click();
        }),
        );


    }

    fn on_win_click(&self) {
        let imp = self.imp();
        imp.window_flap.set_reveal_flap(!imp.window_flap.reveals_flap());
    }

    pub fn update_image(&self, image_file: &gio::File){
        let imp = self.imp();

        imp.main_image.set_file(Some(image_file));

        let image_name = image_file.query_info(&gio::FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME, gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE);

        let image_name = match image_name {
            Ok(name) => name,
            Err(error) => {
                return
            }
        };


        imp.window_title.set_title(&image_name.display_name())
    }



}

impl Default for AdwaifehWindow {
    fn default() -> Self {
        AdwaifehApplication::default()
            .active_window()
            .unwrap()
            .downcast()
            .unwrap()
    }
}

