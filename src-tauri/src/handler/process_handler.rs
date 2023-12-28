use std::sync::OnceLock;

use parking_lot::Mutex;

use crate::utils::process;
use crate::utils::process::focus_previous_process_window;

pub struct ProcessHandler {
    pub previous_process_id: Mutex<i32>,
}

unsafe impl Sync for ProcessHandler {}

impl ProcessHandler {
    pub fn set_previous_process_id(new_previous_process_id: i32) {
        let mut previous_process_id = Self::global().previous_process_id.lock();
        *previous_process_id = new_previous_process_id;
    }

    pub fn get_previous_process_id() -> i32 {
        *Self::global().previous_process_id.lock()
    }

    pub fn paste_in_previous_window() {
        let process_id = Self::get_previous_process_id();
        println!("paste to previous process id: {}", process_id);
        if process_id > 0 {
            focus_previous_process_window(process_id);
            process::paste();
        }
    }

    pub fn focus_previous_window() {
        let process_id = Self::get_previous_process_id();
        if process_id > 0 {
            focus_previous_process_window(process_id);
        }
    }

    fn global() -> &'static ProcessHandler {
        static HANDLE: OnceLock<ProcessHandler> = OnceLock::new();

        HANDLE.get_or_init(|| ProcessHandler {
            previous_process_id: Mutex::new(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::process_handler::ProcessHandler;
    use crate::utils::process::get_active_process_id;

    #[test]
    fn test_set_get_previous_process_id() {
        assert_eq!(ProcessHandler::get_previous_process_id(), 0);
        let process_id = get_active_process_id();
        ProcessHandler::set_previous_process_id(process_id);
        assert_eq!(ProcessHandler::get_previous_process_id(), process_id);
    }
}
