// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aquadoggo::Node;
use dotenv::dotenv;
use log::{error, info};
use tauri::async_runtime::block_on;
use tauri::{App, Manager};
use window_shadows::set_shadow;

use app::consts::MAIN_WINDOW;
use app::p2panda::node::NodeServer;
use app::tray::register_tray;
use app::{handler, listener, logger};

fn main() {
    dotenv().ok();
    logger::init();

    // Step 0: Create and setup application
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app::command::config::graphql_endpoint,
            app::command::record::tap_change_clipboard,
            app::command::record::delete_record,
            app::command::record::toggle_favorite_record,
        ])
        .setup(|app| {
            if cfg!(not(target_os="linux")) {
                let window = app.get_window(MAIN_WINDOW).unwrap();
                if let Err(_) = set_shadow(&window, true) {
                    error!("Set window shadow failed, unsupported platform!")
                }
            }
            setup_service(app);
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
    app.run(|_app_handle, event| match event {
        // Keep the Backend Running in the Background
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::Ready => {
            info!("Application launched!");
        }
        _ => {}
    });
}

fn setup_service(app: &mut App) {
    // Make the docker NOT to have an active app when started
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    // Save application handler
    handler::global_handler::GlobalHandler::global().init(app.app_handle());

    // Start sync server
    let mut node: Option<Node> = None;
    block_on(async {
        node = Some(NodeServer::start().await.unwrap());
    });
    if let Some(node) = node {
        tauri::async_runtime::spawn(async move {
            node.on_exit().await;
        });
    } else {
        error!("Start node server failed!")
    }

    listener::register_all_listeners(app).unwrap();
}
