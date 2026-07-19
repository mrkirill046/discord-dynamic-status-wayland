mod hyprland;
mod niri;

use crate::config::Settings;
use anyhow::{Result, bail};

pub fn listen_active_window<F>(settings: &Settings, handler: F) -> Result<()>
where
    F: FnMut(String, String) -> Result<()>,
{
    match settings.wm.as_str() {
        "hyprland" => hyprland::listen_active_window(handler),
        "niri" => niri::listen_active_window(handler),

        wm => {
            bail!("Unsupported WM: {}", wm)
        }
    }
}
