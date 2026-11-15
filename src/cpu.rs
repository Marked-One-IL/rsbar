use gtk4 as gtk;
use gtk::prelude::*;
use crate::info;

pub fn cpu(bar: &gtk::Box) {
        let label = gtk::Label::new(Some(&info::cpu()));
        label.add_css_class("cpu");
        
        label.set_margin_start(12);
        label.set_margin_end(12);

        bar.append(&label);
        gtk::glib::timeout_add_local(
            std::time::Duration::from_secs(1),
            move || {
                label.set_text(&info::cpu());
                gtk::glib::ControlFlow::Continue
            },
        );

}
