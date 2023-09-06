// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use log::info;
use tauri::{App, Manager};

use crate::listener::clipboard::ClipboardListener;
use crate::listener::global_event_listener::GlobalEventListener;
use crate::tray::register_tray;

mod config;
mod consts;
mod dao;
mod handler;
mod listener;
mod logger;
mod models;
mod schema;
mod storage;
mod tray;
mod utils;

fn main() {
    dotenv().ok();
    logger::init();

    // Step 0: Create and setup application
    let app = tauri::Builder::default()
        .setup(|app| {
            setup(app);
            Ok(())
        });

    // Step 1: register system tray
    let app = register_tray(app);

    // Step 2:

    // Step 3: build application
    let app = app
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // Step 4: run application
    info!("Application launched!");
    app.run(|_app_handle, event| match event {
        // Keep the Backend Running in the Background
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn setup(app: &mut App) {
    // Make the docker NOT to have an active app when started
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    // Save application handler
    handler::global_handler::GlobalHandler::global().init(app.app_handle());

    // Start global application listener
    GlobalEventListener::register_all_global_listeners(app).unwrap();

    // Start listening for clipboard
    ClipboardListener::listen();
}
