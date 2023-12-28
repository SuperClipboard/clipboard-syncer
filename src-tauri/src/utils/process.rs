use std::{thread, time};

use active_win_pos_rs::get_active_window;
use log::error;
use rdev::{simulate, EventType, Key, SimulateError};

pub fn get_active_process_id() -> i32 {
    match get_active_window() {
        Ok(active_window) => {
            let process_id: i32 = active_window.process_id.try_into().unwrap();
            process_id
        }
        Err(()) => {
            error!("error occurred while getting the active window");
            0
        }
    }
}

pub fn focus_previous_process_window(process_id: i32) {
    if process_id == 0 {
        return;
    }
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::appkit::NSApplicationActivateIgnoringOtherApps;
        use cocoa::appkit::NSRunningApplication;
        let current_app = NSRunningApplication::runningApplicationWithProcessIdentifier(
            cocoa::base::nil,
            process_id,
        );
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }
}

pub fn paste() {
    // Run command + v to paste
    // Same approach as both Maccy and Clipy, reference: https://github.com/p0deje/Maccy/blob/master/Maccy/Clipboard.swift#L101
    dispatch(EventType::KeyPress(Key::MetaLeft));
    dispatch(EventType::KeyPress(Key::KeyV));
    dispatch(EventType::KeyRelease(Key::KeyV));
    dispatch(EventType::KeyRelease(Key::MetaLeft));
}

pub fn request_permissions() {
    // Simply press and release the shift key. First time the OS will ask for permissions, then do it without asking.
    dispatch(EventType::KeyPress(Key::ShiftLeft));
    dispatch(EventType::KeyRelease(Key::ShiftLeft));
}

fn dispatch(event_type: EventType) {
    match simulate(&event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not dispatch {:?}", event_type);
        }
    }
    // Let the OS catchup (at least MacOS)
    sleep(20)
}

fn sleep(ms: u64) {
    let delay = time::Duration::from_millis(ms);
    thread::sleep(delay);
}

#[cfg(test)]
mod tests {
    use crate::utils::process::get_active_process_id;

    #[test]
    fn test_get_active_process_id() {
        println!("current active process id: {:?}", get_active_process_id())
    }
}
