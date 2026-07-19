use crate::logger::Logger;
use anyhow::{Context, Result};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn listen_active_window<F>(mut handler: F) -> Result<()>
where
    F: FnMut(String, String) -> Result<()>,
{
    if std::env::var_os("NIRI_SOCKET").is_none() {
        anyhow::bail!("NIRI_SOCKET is not set. Are you running inside Niri?");
    }

    let mut child = Command::new("niri")
        .args(["msg", "event-stream"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to start niri event stream")?;

    let stdout = child.stdout.take().context("Failed to read niri stdout")?;
    let reader = BufReader::new(stdout);

    Logger::debug("Started Niri event stream");

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("Window focus changed:") {
            update_focused_window(&mut handler)?;
        }
    }

    Ok(())
}

fn update_focused_window<F>(handler: &mut F) -> Result<()>
where
    F: FnMut(String, String) -> Result<()>,
{
    let output = Command::new("niri")
        .args(["msg", "-j", "windows"])
        .output()
        .context("Failed to get niri windows")?;

    let windows: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let windows = windows.as_array().context("Invalid windows json")?;

    for window in windows {
        if window["is_focused"].as_bool().unwrap_or(false) {
            let class = window["app_id"].as_str().unwrap_or("").to_string();
            let title = window["title"].as_str().unwrap_or("").to_string();

            Logger::debug(&format!("Current class: {}, title: {}", class, title));

            handler(class, title)?;

            break;
        }
    }

    Ok(())
}
