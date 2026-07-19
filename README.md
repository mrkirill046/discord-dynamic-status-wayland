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
* Supports any application (settings in `config.json`)
* Lightweight and fast (Rust)
* Works on Hyprland & Niri (Wayland)
* NixOS is supported

---

## 🏗 Installation and Run

### 1. Clone repository
> Or use `yay -S dynamic-drpc-wayland-bin` or `yay -S dynamic-drpc-wayland-git`
```bash
git clone https://github.com/mrkirill046/discord-dynamic-status-wayland.git
cd discord-dynamic-status-wayland
cargo run --release
```

### 2. Configure `config.json` (in the `~/.local/share/dynamic-drpc-wayland`)

```json
{
  "app_id": "1460605258072985705 (Done by me)",
  "default": {
    "state": "Chilling",
    "details": "At the workspace",
    "large_text": "NixOS 26.11 (Zokor)",
    "large_image": "nixos",
    "small_text": "Niri (Wayland)",
    "small_image": "niri"
  },
  "classes": {
    "com.mitchellh.ghostty": {
      "state": "At ghostty",
      "details": "Writing command lines",
      "large_text": "NixOS 26.11 (Zokor)",
      "large_image": "nixos",
      "small_text": "Ghostty with Fish",
      "small_image": "ghostty"
    }
  }
} // etc
```

> **Important:** use your **Application ID** from Discord Developer Portal, **not a bot token**.

* I have also already added all images in the current default config using my App ID

---

## 🛠 Troubleshooting

* Discord must be **online** and **not in Invisible** mode
* All assets in `config.json` must exist in Discord Developer Portal → Art Assets
* Of course, you should have an internet connection :)

---

## 📝 License

MIT License — see [LICENSE](LICENSE)
