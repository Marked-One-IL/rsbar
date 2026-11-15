use gtk4 as gtk;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
mod info;
mod cpu;
mod clock;
fn main() -> gtk::glib::ExitCode {
    const HEIGHT: i32 = 32;
    const WIDTH: i32 = 800;
    const MARGINLEFT: i32 = 8;
    const MARGINRIGHT: i32 = 8;
    const MARGINCENTER: i32 = 8;
    const MARGINTOP: i32 = 8;
    const MARGINBOTTOM: i32 = 8;
    let app = gtk::Application::builder()
        .application_id("rs.bar.me")
        .build();
    app.connect_activate(|app| {
        let provider = gtk::CssProvider::new();
        let css_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/style.css");
        let mut css = std::fs::read_to_string(&css_path)
            .unwrap_or_else(|_| include_str!("style.css").to_owned());
        provider.load_from_data(&css);
        
        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not connect to display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        gtk::glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
            if let Ok(updated_css) = std::fs::read_to_string(&css_path) {
                if updated_css != css {
                    provider.load_from_data(&updated_css);
                    css = updated_css;
                }
            }
            gtk::glib::ControlFlow::Continue
        });

        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .decorated(false)
            .default_height(HEIGHT)
            .build();
        window.init_layer_shell();
        if window.is_anchor(Edge::Top) {
            window.set_margin(Edge::Top, MARGINTOP);
        } else {
            window.set_margin(Edge::Bottom, MARGINBOTTOM);
        }
        window.set_layer(Layer::Top);
        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Right, false);
        window.set_default_size(WIDTH, HEIGHT);

        window.auto_exclusive_zone_enable();

        let bar = gtk::CenterBox::new();
        let left = gtk::Box::new(gtk::Orientation::Horizontal, MARGINLEFT);
        let right = gtk::Box::new(gtk::Orientation::Horizontal, MARGINRIGHT);
        let center = gtk::Box::new(gtk::Orientation::Horizontal, MARGINCENTER);
        bar.set_start_widget(Some(&left));
        bar.set_center_widget(Some(&center));
        bar.set_end_widget(Some(&right));

        bar.set_height_request(HEIGHT);
        cpu::cpu(&left);
        clock::time(&center);


        window.set_child(Some(&bar));

        window.present();
    });
    
    app.run()
}
