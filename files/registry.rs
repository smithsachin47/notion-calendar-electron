//! Windows registry read helpers for Notion Calendar Electron.

pub const APP_KEY: &str = "Software\\Notion\\Notion Calendar Electron";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Notion Calendar Electron";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Notion Calendar Electron") },
        RegValue { name: "Version",     value: String::from("v2.1.17") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Notion Calendar Electron") },
        RegValue { name: "Publisher",       value: String::from("Notion") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Notion Calendar Electron\\uninstall.exe\"") },
    ]
}
