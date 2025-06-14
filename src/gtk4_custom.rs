use gtk::{prelude::*, Label};
use crate::xrandr_binds;
use std::rc::Rc;
use std::cell::RefCell;

pub fn create_warning_window (title: &str,message: &str)-> gtk::Dialog {
    let output_warning = gtk::Dialog::new();
    output_warning.set_default_width(500);

    output_warning.set_title(Some(title));
    let content_area = output_warning.content_area();
    content_area.set_margin_top(20);
    content_area.set_margin_bottom(20);
    content_area.set_margin_start(20);
    content_area.set_margin_end(20);
    
    let label_output_warning = gtk::Label::new(Some(message));
    label_output_warning.set_wrap(true);
    label_output_warning.set_max_width_chars(50); 

    let button_output_warning = gtk::Button::new();
    button_output_warning.set_label("Close");
    button_output_warning.set_margin_top(15);
    button_output_warning.set_hexpand(false);
    button_output_warning.set_halign(gtk::Align::Center);
    button_output_warning.set_vexpand(false);
    button_output_warning.set_valign(gtk::Align::Start);

    button_output_warning.connect_clicked(|_butt| {
        std::process::exit(0);
    });

    content_area.append(&label_output_warning);
    content_area.append(&button_output_warning);

    output_warning
}

pub struct BrightnessScale {
    pub container: gtk::Box,
    pub scale: gtk::Scale,
    pub adjustment: gtk::Adjustment,
    pub value_label: gtk::Label,
    pub screen_name: String,
}

impl BrightnessScale {
    pub fn new(screen_name: &str) -> Self {
        // Declaration.
        let output_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let scale_name_label = gtk::Label::new(Some(screen_name));
        scale_name_label.set_width_chars(10);
        let scalebox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        scalebox.set_hexpand(true);
        scalebox.set_halign(gtk::Align::Fill);
        let adjustment = gtk::Adjustment::new(xrandr_binds::get_brightness(screen_name)*100.0, 0.0, 100.0, 1.0, 10.0, 0.0);
        let scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
        scale.set_hexpand(true);
        scale.set_halign(gtk::Align::Fill);
        let scale_value_label = gtk::Label::new(Some(&adjustment.value().to_string()));
        scale_value_label.set_width_chars(3);

        // Callbacks.
        let screen_name_owned = screen_name.to_string();
        let scale_value_label_clone = scale_value_label.clone();
        adjustment.connect_value_changed(move |adj| {
            scale_value_label_clone.set_text(&adj.value().round().to_string());
            let value = adj.value().round() / 100.0;
            xrandr_binds::set_brightness(&screen_name_owned, &value.to_string());
        });

        // Appends.
        output_box.append(&scale_name_label);
        scalebox.append(&scale);
        scalebox.append(&scale_value_label);
        output_box.append(&scalebox);

        Self {
            container: output_box,
            scale,
            adjustment,
            value_label: scale_value_label,
            screen_name: screen_name.to_string(),
        }
    }

    pub fn set_scale_value(&self, value: f64) {
        self.adjustment.set_value(value);
    }
}

pub fn create_all_brightness_scale(brightness_scales: Vec<BrightnessScale>)-> gtk::Box {
    let brightness_scales = Rc::new(RefCell::new(brightness_scales));
    // Declaration. 

    let output_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let scale_name_label = gtk::Label::new(Some("All monitors"));
    scale_name_label.set_width_chars(10);

    let scalebox= gtk::Box::new(gtk::Orientation::Horizontal, 10);
        scalebox.set_hexpand(true);
        scalebox.set_halign(gtk::Align::Fill);

    let adjustment = gtk::Adjustment::new(100.0, 0.0, 100.0, 1.0, 10.0, 0.0);
    let scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
        scale.set_hexpand(true);
        scale.set_halign(gtk::Align::Fill);
    
    let scale_value_label = gtk::Label::new(Some(&adjustment.value().to_string()));
        scale_value_label.set_width_chars(3);

    // Callbacks. 
    let scale_value_label_clone = scale_value_label.clone();
    
    adjustment.connect_value_changed(move |adj| { 
        scale_value_label_clone.set_text(&adj.value().round().to_string());
        let value = adj.value().round();

        for scale in brightness_scales.borrow().iter() {
            scale.set_scale_value(value);
        }
        
    });

    // Appends.
    
    output_box.append(&scale_name_label);
        scalebox.append(&scale);
        scalebox.append(&scale_value_label);
    output_box.append(&scalebox);

    output_box
}

pub struct GammaScale {
    pub container: gtk::Box,
    pub scale: gtk::Scale,
    pub adjustment: gtk::Adjustment,
    pub value_label: gtk::Label,
    pub screen_name: String,
}

impl GammaScale {
    pub fn new(screen_name: &str) -> Self {
        // Declaration.
        let output_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let scale_name_label = gtk::Label::new(Some(screen_name));
        scale_name_label.set_width_chars(10);
        let scalebox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        scalebox.set_hexpand(true);
        scalebox.set_halign(gtk::Align::Fill);
        let adjustment = gtk::Adjustment::new(0.0, 0.0, 100.0, 1.0, 10.0, 0.0);
        let scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
        scale.set_hexpand(true);
        scale.set_halign(gtk::Align::Fill);
        let scale_value_label = gtk::Label::new(Some(&adjustment.value().to_string()));
        scale_value_label.set_width_chars(3);

        // Callbacks.
        let screen_name_owned = screen_name.to_string();
        let scale_value_label_clone = scale_value_label.clone();
        adjustment.connect_value_changed(move |adj| {
            scale_value_label_clone.set_text(&adj.value().round().to_string());
            let value = adj.value().round() / 100.0;
            // xrandr_binds::set_brightness(&screen_name_owned, &value.to_string());                                                                                           //  1.0 - (S * 0.4 / 100)
            xrandr_binds::set_gamma(&screen_name_owned, &xrandr_binds::get_brightness(&screen_name_owned).to_string(), &value.to_string(), &(1.0 - (value*0.4)).to_string(), &(1.0 - (value*0.8)).to_string());
         });

        // Appends.
        output_box.append(&scale_name_label);
        scalebox.append(&scale);
        scalebox.append(&scale_value_label);
        output_box.append(&scalebox);

        Self {
            container: output_box,
            scale,
            adjustment,
            value_label: scale_value_label,
            screen_name: screen_name.to_string(),
        }
    }

    pub fn set_scale_value(&self, value: f64) {
        self.adjustment.set_value(value);
    }
}

pub fn create_all_gamma_scale(gamma_scales: Vec<GammaScale>)-> gtk::Box {
    let gamma_scales = Rc::new(RefCell::new(gamma_scales));
    // Declaration. 

    let output_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let scale_name_label = gtk::Label::new(Some("All monitors"));
    scale_name_label.set_width_chars(10);

    let scalebox= gtk::Box::new(gtk::Orientation::Horizontal, 10);
        scalebox.set_hexpand(true);
        scalebox.set_halign(gtk::Align::Fill);

    let adjustment = gtk::Adjustment::new(100.0, 0.0, 100.0, 1.0, 10.0, 0.0);
    let scale = gtk::Scale::new(gtk::Orientation::Horizontal, Some(&adjustment));
        scale.set_hexpand(true);
        scale.set_halign(gtk::Align::Fill);
    
    let scale_value_label = gtk::Label::new(Some(&adjustment.value().to_string()));
        scale_value_label.set_width_chars(3);

    // Callbacks. 
    let scale_value_label_clone = scale_value_label.clone();
    
    adjustment.connect_value_changed(move |adj| { 
        scale_value_label_clone.set_text(&adj.value().round().to_string());
        let value = adj.value().round();

        for scale in gamma_scales.borrow().iter() {
            scale.set_scale_value(value);
        }
        
    });

    // Appends.
    
    output_box.append(&scale_name_label);
        scalebox.append(&scale);
        scalebox.append(&scale_value_label);
    output_box.append(&scalebox);

    output_box
}


pub fn create_brightness_page() -> gtk::Box {
    let output_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        output_box.set_margin_top(20);
        output_box.set_margin_bottom(20);
        output_box.set_margin_start(20);
        output_box.set_margin_end(20);

    let mut brightness_scales: Vec<BrightnessScale> = vec![];

    for monitor in xrandr_binds::list_enable_monitors() {
        let brightness_scale = BrightnessScale::new(&monitor);
        output_box.append(&brightness_scale.container);
        brightness_scales.push(brightness_scale);
    }

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    output_box.append(&separator);

    output_box.append(&create_all_brightness_scale(brightness_scales));

    
    output_box
}

pub fn create_gamma_page() -> gtk::Box {
    let output_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        output_box.set_margin_top(20);
        output_box.set_margin_bottom(20);
        output_box.set_margin_start(20);
        output_box.set_margin_end(20);

    let mut gamma_scales: Vec<GammaScale> = vec![];

    for monitor in xrandr_binds::list_enable_monitors() {
        let gamma_scale = GammaScale::new(&monitor);
        output_box.append(&gamma_scale.container);
        gamma_scales.push(gamma_scale);
    }

    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    output_box.append(&separator);
    output_box.append(&create_all_gamma_scale(gamma_scales));

    let label = gtk::Label::new(Some("The display of gamma values will not be updated due to problems involving xrandr and video drivers."));
    label.set_wrap(true);
    output_box.append(&label);

    output_box
}