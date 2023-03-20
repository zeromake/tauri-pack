// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{utils::config::AppUrl, WindowUrl};
use std::env;

fn main() {
    let mut context = tauri::generate_context!();
    // 从本地文件加载来跳过 https 无法请求 http 和 ws 的问题。
    let current_exe = env::current_exe().unwrap();
    let path = current_exe.parent().unwrap();
    let url = format!("file://{}/_up_/dist/index.html", path.display()).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
    tauri::Builder::default()
        .run(context)
        .expect("error while running tauri application");
}
