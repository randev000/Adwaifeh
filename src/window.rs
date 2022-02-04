use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, gdk, CompositeTemplate};
use libadwaita::subclass::prelude::*;
use glib::{clone};
use std::time::Duration;
use crate::AdwaifehApplication;



mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/randev/Adwaifeh/window.ui")]
    pub struct AdwaifehWindow {


        // Template widgets
        #[template_child]
        pub main_image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub header_revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub header_overlay: TemplateChild<gtk::Overlay>,
        #[template_child]
        pub headerbar: TemplateChild<libadwaita::HeaderBar>,
        #[template_child]
        pub mouse_handler: TemplateChild<gtk::EventControllerMotion>,
        #[template_child]
        pub click_handler: TemplateChild<gtk::GestureClick>,
        #[template_child]
        pub drag_handler: TemplateChild<gtk::GestureDrag>,
        #[template_child]
        pub window_title: TemplateChild<libadwaita::WindowTitle>,

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

    let headerbar = &*imp.headerbar;
    let header_revealer = &*imp.header_revealer;

    let height = headerbar.height();
    let width = headerbar.height();

    header_revealer.set_size_request(width, height);

    print!("{}", header_revealer.height());
    print!("{}", headerbar.height());

    imp.mouse_handler.connect_enter(clone!(@weak self as win => move |_, _, _| {
        win.on_headerbar_mouse_enter();
    }),
    );

    imp.mouse_handler.connect_motion(clone!(@weak self as win => move |_, _, _| {
        win.on_headerbar_mouse_enter();
    }),
    );

    imp.mouse_handler.connect_leave(clone!(@weak self as win => move |_| {
        win.on_headerbar_mouse_leave();
    }),
    );

    imp.click_handler.connect_released(clone!(@weak self as win => move |_, _, _, _| {
        win.on_headerbar_mouse_enter();
    }),
    );

    imp.drag_handler.connect_begin(clone!(@weak self as win => move |_, _| {
        win.on_headerbar_mouse_enter();
    }),
    );

    imp.drag_handler.connect_end(clone!(@weak self as win => move |_, _| {
        win.on_headerbar_mouse_enter();
    }),
    );


    }

    fn on_headerbar_mouse_enter(&self) {
        let imp = self.imp();
        if self.is_active(){
            imp.header_revealer.set_reveal_child(true);
        }
    }

    fn on_headerbar_mouse_leave(&self) {
        let imp = self.imp();

        imp.header_revealer.set_reveal_child(false);

    }

    pub fn update_image(&self, image_file: &gio::File){
        let imp = self.imp();
        imp.headerbar.set_css_classes(&["osd"]);
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

