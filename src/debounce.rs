use crate::logger::Logger;
use anyhow::Result;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

pub struct Debouncer {
    sender: Sender<(String, String)>,
}

impl Debouncer {
    pub fn new<F>(delay: Duration, mut callback: F) -> Self
    where
        F: FnMut(String, String) -> Result<()> + Send + 'static,
    {
        let (tx, rx): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();

        Logger::info(&format!("Debouncer started (delay: {:?})", delay));

        thread::spawn(move || {
            let mut last_sent = String::new();

            Logger::debug("Debounce worker thread started");

            while let Ok((class, title)) = rx.recv() {
                let current = format!("{}:{}", class, title);

                if current == last_sent {
                    Logger::debug("[RPC] Ignoring duplicate window event");
                    continue;
                }

                Logger::debug(&format!("[RPC] Waiting {:?} before update...", delay));

                thread::sleep(delay);

                let mut latest = (class, title);

                while let Ok(event) = rx.try_recv() {
                    latest = event;
                }

                let latest_id = format!("{}:{}", latest.0, latest.1);

                if latest_id == last_sent {
                    Logger::debug("[RPC] Latest window already sent, skipping update");

                    continue;
                }

                Logger::info(&format!(
                    "[RPC] Updating: class='{}', title='{}'",
                    latest.0, latest.1
                ));

                match callback(latest.0.clone(), latest.1.clone()) {
                    Ok(_) => {
                        Logger::info("[RPC] Update successful");
                    }

                    Err(e) => {
                        Logger::error(&format!("Debounce callback error: {:#}", e));
                    }
                }

                last_sent = latest_id;
            }

            Logger::error("Debouncer channel closed, worker stopped");
        });

        Self { sender: tx }
    }

    pub fn send(&self, class: String, title: String) {
        if let Err(e) = self.sender.send((class, title)) {
            Logger::error(&format!("Failed to send debounce event: {}", e));
        }
    }
}
