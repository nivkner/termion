use std::os::windows::io::AsRawHandle;
use std::{io, fs};
use super::winapi::um::handleapi::INVALID_HANDLE_VALUE;
use super::winapi::um::winuser::GetWindowThreadProcessId;
use super::winapi::um::processenv::GetStdHandle;
use super::winapi::um::consoleapi::GetConsoleMode;
use super::winapi::um::processthreadsapi::GetCurrentProcessId;

pub fn is_tty<T: AsRawHandle>(stream: &T) -> bool {
    let std_handle = stream.as_raw_handle();
    if !(std_handle == INVALID_HANDLE_VALUE) {
        let mut dummy_mode = 0;
        unsafe { GetConsoleMode(std_handle, &mut dummy_mode) != 0 }
    } else {
        false
    }
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    unimplemented!();
}
