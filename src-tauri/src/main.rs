// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri_api::dialog::{self, Response};

use markdown::{to_html_with_options, Options};

use std::fs::{read_to_string, write};

#[tauri::command(rename_all = "snake_case")]
fn md_to_html(content: &str) -> String {
    to_html_with_options(content, &Options::gfm()).unwrap()
}

#[derive(serde::Serialize, Clone)]
struct FileResponse {
    contents: String,
    path: String,
}

#[tauri::command(rename_all = "snake_case")]
fn open_file() -> Result<FileResponse, String> {
    match dialog::select(Some(""), Some("")) {
        Ok(res) => match res {
            Response::Okay(path) => Ok(FileResponse {
                contents: read_file(&path),
                path,
            }),
            _ => Err("Couldn't read file!".to_string()),
        },
        Err(e) => Err(format!("{e}")),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn save_file(path: &str, content: &str) -> Result<(), String> {
    write(path, content).map_err(|e| format!("{e}"))
}

#[tauri::command(rename_all = "snake_case")]
async fn new_file() -> Result<FileResponse, String> {
    if let Some(pb) = FileDialogBuilder::new()
        .add_filter("Markdown", &["md"])
        .save_file()
    {
        let path = pb.to_str().unwrap().to_string();
        Ok(FileResponse {
            contents: String::new(),
            path,
        })
    } else {
        Err("Couldn't create file!".to_string())
    }
}

fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            md_to_html, open_file, save_file, new_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
