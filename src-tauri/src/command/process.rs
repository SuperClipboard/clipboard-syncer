use crate::handler::process_handler::ProcessHandler;

#[tauri::command]
pub fn focus_previous_window() -> Result<(), String> {
    ProcessHandler::focus_previous_window();
    Ok(())
}

#[tauri::command]
pub fn paste_in_previous_window() -> Result<(), String> {
    ProcessHandler::paste_in_previous_window();
    Ok(())
}
