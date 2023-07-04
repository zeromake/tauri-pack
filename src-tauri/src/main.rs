// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{utils::config::AppUrl, WindowUrl};
use std::env;
use std::path::Path;
use std::fs;
use toml;
use serde::Serialize;
use serde::Deserialize;
use url::Url;

static USE_HTTP: bool = true;
static USE_LOCAL: bool = false;

#[derive(Serialize, Deserialize)]
struct Config {
   port: u16,
}

#[derive(Serialize, Deserialize)]
struct Data {
    config: Config,
}

fn main() {
    let mut context = tauri::generate_context!();
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build());
    if USE_HTTP {
        let config_dir = tauri::api::path::app_config_dir(context.config()).unwrap();
        let cfg_path = format!("{}/config.toml", config_dir.display());
        let mut port: u16 = 0;
        let mut conf = Data{
            config: Config { port: 0 }
        };
        let config_path = Path::new(&cfg_path);
        if config_path.exists() {
            let contents = fs::read_to_string(cfg_path.clone()).unwrap();
            conf = toml::from_str(&contents).unwrap();
            port = conf.config.port;
        }
        if !config_path.parent().unwrap().exists() {
            fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        }
        if port == 0 || !portpicker::is_free(port)  {
            port = portpicker::pick_unused_port().expect("failed to find unused port");
            conf.config.port = port;
            fs::write(cfg_path, toml::to_string(&conf).unwrap()).unwrap();
        }
        let url = format!("http://localhost:{}/", port).parse().unwrap();
        let window_url = WindowUrl::External(url);
        context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
        builder = builder
            .plugin(tauri_plugin_localhost::Builder::new(port).build())
    } else if USE_LOCAL {
        let current_exe = env::current_exe().unwrap();
        let path = current_exe.parent().unwrap();
        // 从本地文件加载来跳过 https 无法请求 http 和 ws 的问题。
        let url: Url = format!("file://{}/_up_/dist/ariang/dist/index.html", path.display()).parse().unwrap();
        let window_url = WindowUrl::External(url.clone());
        // rewrite the config so the IPC is enabled on this URL
        context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
        context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
    }
    builder
        .run(context)
        .expect("error while running tauri application");
}
