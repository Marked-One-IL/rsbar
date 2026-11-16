use gtk4 as gtk;
use gtk::prelude::*;
use std::process::Command;

pub fn workspaces(bar: &gtk::Box) {
    for tag in 1..=9 {
        let button = gtk::Button::with_label(&tag.to_string());
        button.add_css_class("workspace");

        button.connect_clicked(move |_| {
            if let Err(error) = Command::new("mmsg")
                .args(["dispatch", &format!("view,{tag},0")])
                .spawn()
            {
                eprintln!("Could not switch workspace: {error}");
            }
        });

        bar.append(&button);
    }
}
