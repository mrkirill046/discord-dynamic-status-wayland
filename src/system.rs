pub struct SystemInfo {
    pub pretty_os: String,
    pub os: String,
}

use std::fs;

pub fn get_system_info() -> SystemInfo {
    let data = fs::read_to_string("/etc/os-release").unwrap_or_default();

    let mut pretty_os = "Linux".to_string();
    let mut os = "linux".to_string();

    for line in data.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            pretty_os = value.trim_matches('"').to_string();
        }

        if let Some(value) = line.strip_prefix("ID=") {
            os = value.trim_matches('"').to_string();
        }
    }

    SystemInfo { pretty_os, os }
}
