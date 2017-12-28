use std::{io, mem};
use std::os::windows::io::AsRawHandle;
use super::winapi::um::wincon::GetConsoleScreenBufferInfo;

/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {
    let stdout_handle = io::stdout().as_raw_handle();

    unsafe {
        let mut csbi = mem::zeroed();
        if GetConsoleScreenBufferInfo(stdout_handle, &mut csbi) != 0 {
            Ok(((csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16,
                (csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, format!("Unable to get the terminal size: {}", io::Error::last_os_error())))
        }
    }
}
