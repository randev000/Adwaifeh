mod application;
mod config;
mod window;

use self::application::AdwaifehApplication;
use self::window::AdwaifehWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

fn main() {
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    gtk::init().expect("Failed to initialize gtk");
    libadwaita::init();


    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/adwaifeh.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);



    // Set the name shown in desktop environments
    // glib::set_application_name("Adwaifeh");
    // glib::set_program_name(Some("adwaifeh"));

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    glib::set_application_name("Adwaifeh");
    glib::set_program_name(Some("adwaifeh"));

    let app = AdwaifehApplication::new();
    let ret = app.run();

    std::process::exit(ret);
}
