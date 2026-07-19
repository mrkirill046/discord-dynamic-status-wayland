# `discord-dynamic-status-wayland` 

![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge\&logo=rust)
![License MIT](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)
![Platform: Linux](https://img.shields.io/badge/Platform-Linux-blue?style=for-the-badge) 
![WM](https://img.shields.io/badge/WM-Hyprland+NIRI-purple?style=for-the-badge)

**Dynamic Discord Rich Presence for Hyprland & Niri (Wayland).**
Automatically updates your Discord status based on the active window.

---

## ✨ Features

* Shows the current active window in Discord
* Supports any application (settings in `config.toml`)
* Lightweight and fast (Rust)
* Works on Hyprland & Niri (Wayland)
* NixOS and any arch-like distributive are supported

---

## 🎮 Run

### 1. Clone repository

```bash
git clone https://github.com/mrkirill046/discord-dynamic-status-wayland.git
cd discord-dynamic-status-wayland
cargo run --release
```

### 2. Configure `config.toml` (in the `~/.local/share/dynamic-drpc-wayland`)

```toml
[settings]

# Discord Application ID
# Get it from: Discord Developer Portal -> Applications -> Your App -> General Information
app_id = "1460605258072985705"


# Current Wayland compositor / window manager
# Supported values:
# - niri
# - hyprland
wm = "niri"


# Delay before updating Discord RPC after changing window
# Helps avoid too many RPC updates
update_delay = 3



# Default Rich Presence rule
# This rule is used when no application-specific rule matches
[default]

# Text shown as the first line of Discord Rich Presence
state = "Chilling"

# Text shown as the second line of Discord Rich Presence
details = "At the workspace"


# Large image displayed in Discord
# Supports templates:
# {pretty_os} - full OS name (example: "NixOS 26.11 (Zokor)")
# {os}        - OS identifier (example: "nixos")
large_text = "{pretty_os}"
large_image = "{os}"


# Small image displayed in Discord
# {wm} will be replaced with current window manager name
small_text = "{wm}"
small_image = "{wm}"



# Application-specific Rich Presence rule
# Section name is used as the Discord asset name
# Example:
# [ghostty] -> small_image = "ghostty"
#
# "match" is the real Wayland application class
# You can find it using tools like:
# - `niri msg windows`
# - `hyprctl clients`
[classes.ghostty]

# Wayland class used to detect the application.
# By default, it uses the section name (`classes."name"`).
match = "com.mitchellh.ghostty"


# Custom Rich Presence for Ghostty
state = "At ghostty"
details = "Writing command lines"

# You can also override another default settings
# If a setting is not specified here, the value from the default section will be used
small_text = "Ghostty with Fish"
```

> **Important:** use your **Application ID** from Discord Developer Portal, **not a bot token**.

* I have also already added all images in the current default config using my App ID

---

## 🏗 Installation

### Directly build with Rust using Git

#### 1. Clone repository

```bash
git clone https://github.com/mrkirill046/discord-dynamic-status-wayland.git
cd discord-dynamic-status-wayland
```

#### 2. Build the app

```bash
cargo build --release
```

#### 3. Copy executable to local bin

```bash
cp target/release/discord-dynamic-status-wayland ~/.local/bin/
chmod +x ~/.local/bin/discord-dynamic-status-wayland
```

#### 4. Use or create a service for systemd or whatever you use

```bash 
discord-dynamic-status-wayland 
```

### AUR

> Use `yay -S dynamic-drpc-wayland-bin` or `yay -S dynamic-drpc-wayland-git`

### NixOS

> In the future

---

## 🛠 Troubleshooting

* Discord must be **online** and **not in Invisible** mode
* All assets in `config.toml` must exist in Discord Developer Portal → Art Assets
* Of course, you should have an internet connection :)

---

## 📝 License

MIT License — see [LICENSE](LICENSE)
