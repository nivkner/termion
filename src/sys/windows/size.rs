use std::{io, ptr};
use std::os::windows::io::AsRawHandle;
use sys::tty::get_write_tty;

use super::winapi::um::wincon::GetConsoleScreenBufferInfo;

/// Get the size of the terminal.
pub fn terminal_size() -> io::Result<(u16, u16)> {
    let console_pointer = ptr::null_mut();
	if unsafe { GetConsoleScreenBufferInfo(io::stdout().as_raw_handle(), console_pointer) != 0 } {
        unsafe {console_pointer.as_ref()}
			.map(|info| (info.dwSize.X as u16, info.dwSize.Y as u16))
			.ok_or(io::Error::last_os_error())
    } else {
       Err(io::Error::last_os_error())
    }
}
