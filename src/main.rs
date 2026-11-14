use gtk4 as gtk;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
mod info;
fn main() -> gtk::glib::ExitCode {
    const HEIGHT: i32 = 32;
    let app = gtk::Application::builder()
        .application_id("rs.bar.me")
        .build();
    app.connect_activate(|app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .decorated(false)
            .default_height(HEIGHT)
            .build();
        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.auto_exclusive_zone_enable();

        let mut info = info::Inst::new();
        let bar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        bar.set_height_request(HEIGHT);

        let label = gtk::Label::new(Some(&format!("Cpu: {:.2}%", info.get_cpu_percentage())));
        label.add_css_class("cpu");
        
        label.set_margin_start(12);
        
        label.set_margin_end(12);
        

        bar.append(&label);
        gtk::glib::timeout_add_local(
            std::time::Duration::from_secs(1),
            move || {
                label.set_text(&format!(
                    "Cpu: {:.2}%",
                    info.get_cpu_percentage()
                ));
                gtk::glib::ControlFlow::Continue
            },
        );

        window.set_child(Some(&bar));

        window.present();
    });
    
    app.run()
}
