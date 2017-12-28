use std::{fs, io};
use std::os::unix::io::AsRawFd;

use super::libc;


/// Is this stream a TTY?
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    unsafe { libc::isatty(stream.as_raw_fd()) == 1 }
}

/// Get a read-only file representing the TTY.
///
/// This allows for reading from the TTY if one is available, even when stdin is redirected.
pub fn get_read_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(false).open("/dev/tty")
}

/// Get a write-only file representing the TTY.
///
/// This allows for writing to the TTY if one is available, even when stdout is redirected.
pub fn get_write_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(false).write(true).open("/dev/tty")
}
