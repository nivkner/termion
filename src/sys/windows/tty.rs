use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::os::windows::ffi::OsStrExt;
use std::{io, fs, ptr, ffi};
use super::winapi::um::handleapi::INVALID_HANDLE_VALUE;
use super::winapi::um::winnt::{GENERIC_READ, GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE};
use super::winapi::um::fileapi::{CreateFile2, OPEN_EXISTING};
use super::winapi::um::consoleapi::{GetConsoleMode};

/// Is this stream a TTY?
pub fn is_tty<T: AsRawHandle>(stream: &T) -> bool {
    let std_handle = stream.as_raw_handle();
    if !(std_handle == INVALID_HANDLE_VALUE) {
        let mut dummy_mode = 0;
        unsafe { GetConsoleMode(std_handle, &mut dummy_mode) != 0 }
    } else {
        false
    }
}

/// Get a read-only file representing the TTY.
///
/// This allows for reading from the TTY if one is available, even when stdin is redirected.
pub fn get_read_tty() -> io::Result<fs::File> {
    let handle = unsafe {CreateFile2( ffi::OsStr::new("CONIN$")
                                      .encode_wide()
                                      .chain(Some(0).into_iter())
                                      .collect::<Vec<_>>()
                                      .as_ptr(),
                                      GENERIC_READ,
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

/// Get a write-only file representing the TTY.
///
/// This allows for writing to the TTY if one is available, even when stdout is redirected.
pub fn get_write_tty() -> io::Result<fs::File> {
    let handle = unsafe {CreateFile2( ffi::OsStr::new("CONOUT$")
                                      .encode_wide()
                                      .chain(Some(0).into_iter())
                                      .collect::<Vec<_>>()
                                      .as_ptr(),
                                      GENERIC_WRITE,
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
