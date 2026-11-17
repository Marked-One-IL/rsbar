use gtk::prelude::*;
use gtk4 as gtk;
use std::io::BufReader;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
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

    // Keep only the latest event, so a busy UI cannot accumulate stale states.
    let latest = Arc::new(Mutex::new(None));
    let child = Arc::new(Mutex::new(None::<std::process::Child>));
    let stopped = Arc::new(AtomicBool::new(false));
    let worker_latest = Arc::clone(&latest);
    let worker_child = Arc::clone(&child);
    let worker_stopped = Arc::clone(&stopped);
    std::thread::spawn(move || {
        while !worker_stopped.load(Ordering::Relaxed) {
            // Hold the lock through spawn so shutdown cannot miss a new child.
            let stdout = {
                let mut slot = worker_child.lock().unwrap();
                if worker_stopped.load(Ordering::Relaxed) {
                    break;
                }
                match Command::new("mmsg")
                    .args(["watch", "all-monitors"])
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(mut process) => {
                        let stdout = process.stdout.take();
                        *slot = Some(process);
                        stdout
                    }
                    Err(error) => {
                        eprintln!("Could not watch workspaces: {error}");
                        None
                    }
                }
            };
            if let Some(stdout) = stdout {
                // Parse consecutive JSON values, including pretty-printed objects.
                let events = serde_json::Deserializer::from_reader(BufReader::new(stdout))
                    .into_iter::<serde_json::Value>();
                for event in events {
                    if worker_stopped.load(Ordering::Relaxed) {
                        break;
                    }
                    let state = match event {
                        Ok(state) => state,
                        Err(error) => {
                            eprintln!("Could not read workspace event: {error}");
                            break;
                        }
                    };
                    let active = state.get("monitors")
                        .and_then(|monitors| monitors.as_array())
                        .and_then(|monitors| monitors.iter().find(|monitor| {
                            monitor.get("active").and_then(|v| v.as_bool()) == Some(true)
                        }))
                        .and_then(|monitor| monitor.get("active_tags"))
                        .and_then(|tags| tags.as_array())
                        .map(|tags| tags.iter().filter_map(|tag| tag.as_u64()).collect::<Vec<_>>());
                    if let Some(active) = active {
                        *worker_latest.lock().unwrap() = Some(active);
                    }
                }
            }
            if let Some(mut process) = worker_child.lock().unwrap().take() {
                let _ = process.kill();
                let _ = process.wait();
            }
            if !worker_stopped.load(Ordering::Relaxed) {
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    });

    gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
        if buttons.iter().all(|button| button.upgrade().is_none()) {
            stopped.store(true, Ordering::Relaxed);
            if let Some(process) = child.lock().unwrap().as_mut() {
                let _ = process.kill();
            }
            return gtk::glib::ControlFlow::Break;
        }
        let active = latest.lock().unwrap().take();
        if let Some(active) = active {
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
