use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::os::windows::ffi::OsStrExt;
use std::{io, fs, ptr, ffi};
use super::winapi::um::handleapi::INVALID_HANDLE_VALUE;
use super::winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE};
use super::winapi::um::fileapi::{CreateFile2, OPEN_EXISTING};
use super::winapi::um::consoleapi::{GetConsoleMode};

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
    let handle = unsafe {CreateFile2( ffi::OsStr::new("CONIN$")
                                      .encode_wide()
                                      .chain(Some(0).into_iter())
                                      .collect::<Vec<_>>()
                                      .as_ptr(),
                                      GENERIC_READ | GENERIC_WRITE,
                                      FILE_SHARE_READ | FILE_SHARE_WRITE,
                                      OPEN_EXISTING,
                                      ptr::null_mut())};
    if handle == INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else {
        let file = unsafe { fs::File::from_raw_handle(handle) };
        Ok(file)
    }
}
