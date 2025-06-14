use gtk::{prelude::*};
use gtk::{glib, Application, ApplicationWindow};
mod xrandr_binds;
mod gtk4_custom;

const APP_ID: &str = "org.gtk_rs.xrandr-brightness";
fn main() -> glib::ExitCode {
    // Create a new applicationz
    let app = Application::builder().application_id(APP_ID).build();
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {

    // Check if it is an X.ORG session. If not, it will throw a warning.

    if !xrandr_binds::is_xorg_session () {
        gtk4_custom::create_warning_window("WARNING: X11 IS NOT YOUR COMPOSITING MANAGER", 
        "The XDG_SESSION_TYPE value on your system is not X11. If you want to use xrandr-brightness, please change your compositing manager. The program will not work otherwise. ").present();
    }
   
    // Resolution of the window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("xrandr brightness controller")
        .default_width(600)
        .default_height(300)
        .build();

    // Main box
    let mainbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        //  Vertical and horizontal expansion in window
        mainbox.set_hexpand(true);
        mainbox.set_vexpand(true);
        // "Fill" Alignment 
        mainbox.set_halign(gtk::Align::Fill);
        mainbox.set_valign(gtk::Align::Fill);

    //  Create the Stack that will contain the tabs
    let stack = gtk::Stack::builder()
        .vhomogeneous(false)
        .hhomogeneous(true)
        .build();

    // Creating the StackSwitcher to navigate between tabs
    let stack_switcher = gtk::StackSwitcher::builder()
        .stack(&stack)
        .build();
    let gamma_page = gtk4_custom::create_gamma_page();
    let brightness_page = gtk4_custom::create_brightness_page();
    stack.add_titled(&brightness_page, Some("info"), "Brightness");
    stack.add_titled(&gamma_page, Some("info"), "Gamma");
    
    mainbox.append(&stack_switcher);
    mainbox.append(&stack);

    window.set_child(Some(&mainbox));
    window.present();

}

