use std::io::Write;
use windows_sys::Win32::System::Console::*;

pub fn init_console() {
    unsafe {
        // Allocate a new console
        if AllocConsole() == 0 {
            // AllocConsole failed, maybe one exists already
            //eprintln!("Failed to allocate console");
            return;
        }

        // Redirect stdout
        let stdout = windows_sys::Win32::System::Console::GetStdHandle(STD_OUTPUT_HANDLE);
        if stdout == 0 {
            //eprintln!("Failed to get stdout handle");
        }

        // Optional: redirect Rust stdio macros
        let _ = std::io::stdout().flush();
        //println!("Console allocated!");
    }
}