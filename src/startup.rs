use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupItem {
    pub name: String,
    pub command: String,
    pub location: String,
    pub enabled: bool,
    pub key_path: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn fetch_startup_items() -> Vec<StartupItem> {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut items = Vec::new();

    // 1. HKCU Run
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run") {
        for (name, val) in key.enum_values().flatten() {
            let cmd = val.to_string();
            items.push(StartupItem {
                name,
                command: cmd,
                location: "Registry (HKCU)".to_string(),
                enabled: true,
                key_path: Some("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string()),
            });
        }
    }

    // 2. HKLM Run
    if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_READ) {
        for (name, val) in key.enum_values().flatten() {
            let cmd = val.to_string();
            items.push(StartupItem {
                name,
                command: cmd,
                location: "Registry (HKLM)".to_string(),
                enabled: true,
                key_path: Some("HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string()),
            });
        }
    }

    // 3. User Startup Folder
    if let Ok(appdata) = std::env::var("APPDATA") {
        let startup_dir = std::path::PathBuf::from(appdata)
            .join("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
        if let Ok(entries) = std::fs::read_dir(&startup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    items.push(StartupItem {
                        name,
                        command: path.to_string_lossy().to_string(),
                        location: "Startup Folder".to_string(),
                        enabled: true,
                        key_path: None,
                    });
                }
            }
        }
    }

    items
}

#[cfg(not(target_os = "windows"))]
pub fn fetch_startup_items() -> Vec<StartupItem> {
    Vec::new()
}

pub fn open_file_location(cmd_path: &str) {
    // Extract binary path from command line (strip quotes or flags)
    let clean = cmd_path.trim().trim_matches('"');
    let path_part = clean.split_whitespace().next().unwrap_or(clean).trim_matches('"');
    let p = std::path::Path::new(path_part);
    if p.exists() {
        let _ = std::process::Command::new("explorer")
            .args(["/select,", path_part])
            .spawn();
    } else if let Some(parent) = p.parent() {
        if parent.exists() {
            let _ = std::process::Command::new("explorer")
                .arg(parent)
                .spawn();
        }
    }
}

#[cfg(target_os = "windows")]
pub fn remove_startup_item(item: &StartupItem) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    if item.location.contains("HKCU") {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(key) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_SET_VALUE) {
            if key.delete_value(&item.name).is_ok() {
                return Ok(());
            }
        }
        Err(format!("Failed to delete registry key '{}' from HKCU Run", item.name))
    } else if item.location.contains("HKLM") {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let mut deleted = false;
        if let Ok(key) = hklm.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_SET_VALUE) {
            if key.delete_value(&item.name).is_ok() {
                deleted = true;
            }
        }
        if deleted {
            return Ok(());
        }

        // Elevate via UAC if standard write to HKLM fails
        let status = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Start-Process reg -ArgumentList 'delete \"HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\" /v \"{}\" /f' -Verb RunAs -WindowStyle Hidden",
                    item.name
                ),
            ])
            .status();

        if let Ok(s) = status {
            if s.success() {
                return Ok(());
            }
        }

        Err(format!("Failed to delete registry key '{}' from HKLM Run (access denied)", item.name))
    } else if item.location.contains("Startup Folder") {
        let path = std::path::Path::new(&item.command);
        if path.exists() {
            if std::fs::remove_file(path).is_ok() {
                return Ok(());
            }
            // Elevate via UAC if permission denied
            let status = std::process::Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    &format!(
                        "Start-Process powershell -ArgumentList '-NoProfile -Command Remove-Item -Path \"{}\" -Force' -Verb RunAs -WindowStyle Hidden",
                        item.command
                    ),
                ])
                .status();

            if let Ok(s) = status {
                if s.success() {
                    return Ok(());
                }
            }
        }
        Err(format!("Failed to remove file '{}' from Startup folder", item.name))
    } else {
        Err(format!("Unknown startup item location: {}", item.location))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn remove_startup_item(_item: &StartupItem) -> Result<(), String> {
    Err("Startup management is only supported on Windows".to_string())
}
