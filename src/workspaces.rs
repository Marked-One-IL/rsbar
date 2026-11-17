use gtk::prelude::*;
use gtk4 as gtk;
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;

pub fn workspaces(bar: &gtk::Box) {
    let mut buttons = Vec::new();
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
        buttons.push(button.downgrade());
    }

    // Keep compositor queries off the GTK thread. A bounded channel prevents
    // updates from accumulating if the UI is busy.
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        loop {
            let active = Command::new("mmsg")
                .args(["get", "all-monitors"])
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| serde_json::from_slice::<serde_json::Value>(&output.stdout).ok())
                .and_then(|state| {
                    let monitor = state.get("monitors")?.as_array()?.iter().find(|monitor| {
                        monitor.get("active").and_then(|v| v.as_bool()) == Some(true)
                    })?;
                    Some(
                        monitor
                            .get("active_tags")?
                            .as_array()?
                            .iter()
                            .filter_map(|tag| tag.as_u64())
                            .collect::<Vec<_>>(),
                    )
                });

            match sender.try_send(active) {
                Err(mpsc::TrySendError::Disconnected(_)) => break,
                _ => {}
            }
            std::thread::sleep(Duration::from_millis(250));
        }
    });

    gtk::glib::timeout_add_local(Duration::from_millis(100), move || {
        if buttons.iter().all(|button| button.upgrade().is_none()) {
            return gtk::glib::ControlFlow::Break;
        }
        if let Ok(Some(active)) = receiver.try_recv() {
            for (index, button) in buttons.iter().enumerate() {
                if let Some(button) = button.upgrade() {
                    if active.contains(&(index as u64 + 1)) {
                        button.add_css_class("active");
                    } else {
                        button.remove_css_class("active");
                    }
                }
            }
        }
        gtk::glib::ControlFlow::Continue
    });
}
