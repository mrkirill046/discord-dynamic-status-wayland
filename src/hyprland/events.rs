use crate::logger::Logger;
use anyhow::{Context, Result};
use std::{
    env,
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
};

pub fn listen_active_window<F>(mut handler: F) -> Result<()>
where
    F: FnMut(String, String) -> Result<()>,
{
    let runtime = env::var("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR is missing")?;
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .context("HYPRLAND_INSTANCE_SIGNATURE is missing")?;

    Logger::debug(&format!("Runtime: {}, Signature: {}", runtime, sig));

    let path = format!("{runtime}/hypr/{sig}/.socket2.sock");
    let stream = UnixStream::connect(path).context("Failed to connect to Hyprland socket")?;

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = line.context("Failed to read Hyprland event")?;

        if let Some(data) = line.strip_prefix("activewindow>>") {
            let mut parts = data.splitn(2, ',');

            let class = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Invalid activewindow event"))?
                .to_string();

            let title = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Invalid activewindow event"))?
                .to_string();

            Logger::debug(&format!("Current class: {}, title: {}", class, title));

            handler(class, title)?;
        }
    }

    Ok(())
}
