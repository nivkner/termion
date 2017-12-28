use std::{env, fs, io};
use std::os::unix::io::AsRawFd;

use super::syscall;

/// Is this stream a TTY?
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    if let Ok(fd) = syscall::dup(stream.as_raw_fd(), b"termios") {
        let _ = syscall::close(fd);
        true
    } else {
        false
    }
}

/// Get a read-only file representing the TTY.
///
/// This allows for reading from the TTY if one is available, even when stdin is redirected.
pub fn get_read_tty() -> io::Result<fs::File> {
    let tty = try!(env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)));
    fs::OpenOptions::new().read(true).write(false).open(tty)
}

/// Get a write-only file representing the TTY.
///
/// This allows for writing to the TTY if one is available, even when stdout is redirected.
pub fn get_write_tty() -> io::Result<fs::File> {
    let tty = try!(env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)));
    fs::OpenOptions::new().read(false).write(true).open(tty)
}
