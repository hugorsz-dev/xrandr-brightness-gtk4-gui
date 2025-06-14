use gtk::{prelude::*};
use gtk::{glib, Application, ApplicationWindow};
mod xrandr_binds;
mod gtk4_custom;

// xrandr --output HDMI-0 --gamma 1.0:1.0:1
// xrandr --output HDMI-0 --brightness 0.8 --gamma 1.0:0.7:0.5
// Estaría bien poner un stack
// Colocar una header bar para cargar de nuevo las pantallas... (opcional)
/*
# Escala 0 (sin filtro - luz normal/fría)
xrandr --output <NOMBRE_PANTALLA> --gamma 1.0:1.0:1.0

# Escala 25 (ligeramente cálido)
xrandr --output HDMI-0 --gamma 1.0:0.9:0.8

# Escala 50 (moderadamente cálido)
xrandr --output HDMI-0 --gamma 1.0:0.8:0.6

# Escala 75 (muy cálido)
xrandr --output HDMI-0 --gamma 1.0:0.7:0.4

# Escala 100 (extremadamente cálido)
xrandr --output HDMI-0 --gamma 1.0:0.6:0.2

Rojo = 1.0 (siempre constante)
Verde = 1.0 - (S * 0.4 / 100)
Azul = 1.0 - (S * 0.8 / 100)

*/

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

    if !xrandr_binds::is_xorg_session () {
        gtk4_custom::create_warning_window("WARNING: X11 IS NOT YOUR COMPOSITING MANAGER", 
        "The XDG_SESSION_TYPE value on your system is not X11. If you want to use xrandr-brightness, please change your compositing manager. The program will not work otherwise. ").present();
    }
   
    // Configuración de la resolución de pantalla 
    let window = ApplicationWindow::builder()
        .application(app)
        .title("xrandr brightness controller")
        .default_width(600)
        .default_height(300)
        .build();

    // Caja principal dentro de la ventana
    let mainbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        // Expansión vertical y horizontal en window
        mainbox.set_hexpand(true);
        mainbox.set_vexpand(true);
        // Alineación "fill"
        mainbox.set_halign(gtk::Align::Fill);
        mainbox.set_valign(gtk::Align::Fill);

    // Crear el Stack que contendrá las pestañas
    let stack = gtk::Stack::builder()
        .vhomogeneous(false)
        .hhomogeneous(true)
        .build();

    // Crear el StackSwitcher para navegar entre pestañas
    let stack_switcher = gtk::StackSwitcher::builder()
        .stack(&stack)
        .build();

    let brightness_page = gtk4_custom::create_brightness_page();
    stack.add_titled(&brightness_page, Some("info"), "Brightness");

    let gamma_page = gtk4_custom::create_gamma_page();
    stack.add_titled(&gamma_page, Some("info"), "Gamma");
    
    mainbox.append(&stack_switcher);
    mainbox.append(&stack);

    window.set_child(Some(&mainbox));
    window.present();

}

