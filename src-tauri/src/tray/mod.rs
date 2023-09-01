use tauri::{Builder, Manager, SystemTrayEvent, Wry};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub fn register_tray(app: Builder<Wry>) -> Builder<Wry> {
    app.system_tray(system_tray())
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a left click");
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    // get a handle to the clicked menu item
                    // note that `tray_handle` can be called anywhere,
                    // just get an `AppHandle` instance with `app.handle()` on the setup hook
                    // and move it to another function or thread
                    let item_handle = app.tray_handle().get_item(&id);
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "hide" => {
                            let window = app.get_window("main").unwrap();
                            window.hide().unwrap();
                            // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                            item_handle.set_title("Show").unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
}

fn system_tray() -> SystemTray {
    let tray = SystemTray::new();

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    tray.with_menu(tray_menu)
}
