#[cfg(not(windows))]
use std::{fs, io};

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// Is this stream an TTY?
#[cfg(unix)]
pub fn is_tty<T: AsRawFd>(stream: &T) -> bool {
    use libc;

    unsafe { libc::isatty(stream.as_raw_fd()) == 1 }
}

/// This will panic.
#[cfg(target_os = "redox")]
pub fn is_tty<T: AsRawFd>(_stream: &T) -> bool {
    unimplemented!();
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(target_os = "redox")]
pub fn get_tty() -> io::Result<Box<io::Read>> {
    use std::env;
    let tty = try!(env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)));
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(tty)
        .map(|file| Box::new(file) as Box<io::Read>)
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
#[cfg(unix)]
pub fn get_tty() -> io::Result<Box<io::Read>> {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/tty")
        .map(|file| Box::new(file) as Box<io::Read>)
}

#[cfg(not(windows))]
pub fn init() -> () {
    ()
}

#[cfg(windows)]
// it'll be an api-breaking change to do it later
pub /*(crate)*/ mod windows {
    use kernel32::{GetStdHandle, GetConsoleMode, SetConsoleMode};
    use winapi::wincon::{ENABLE_PROCESSED_OUTPUT, ENABLE_WRAP_AT_EOL_OUTPUT, ENABLE_LINE_INPUT,
                         ENABLE_MOUSE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_QUICK_EDIT_MODE};

    // once winapi 0.3 is available
    // use winapi::wincon::{ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_VIRTUAL_TERMINAL_INPUT};
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
    const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;

    use winapi::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
    use winapi::{FALSE, TRUE};
    use winapi::DWORD;

    use std::io;
    use std::os::windows::prelude::*;

    pub struct PreInitState {
        do_cleanup: bool,
        current_out_mode: DWORD,
        current_in_mode: DWORD,
    }

    impl Drop for PreInitState {
        fn drop(&mut self) {
            if self.do_cleanup {
                unsafe {
                    println!("cleaning up");
                    SetConsoleMode(GetStdHandle(STD_OUTPUT_HANDLE), self.current_out_mode);
                    SetConsoleMode(GetStdHandle(STD_INPUT_HANDLE), self.current_in_mode);
                }
            }
        }
    }

    pub fn init() -> PreInitState {
        // there are many other console hosts on windows that might actually do something
        // rational with the output escape codes, so if the setup fails, carry on rather
        // than reporting an error. The assumption is that the cleanup in the drop trait
        // will always be able to set the flags that are currently set.
        unsafe {
            // TODO: this can fail -> INVALID_HANDLE_VALUE, GetLastError()
            let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            // TODO: this can fail -> INVALID_HANDLE_VALUE, GetLastError()
            let stdin_handle = GetStdHandle(STD_INPUT_HANDLE);

            let mut current_out_mode: DWORD = 0;
            if GetConsoleMode(stdout_handle, &mut current_out_mode) == FALSE {
                return PreInitState {
                    do_cleanup: false,
                    current_out_mode: 0,
                    current_in_mode: 0,
                };
            }

            let mut current_in_mode: DWORD = 0;
            if GetConsoleMode(stdin_handle, &mut current_in_mode) == FALSE {
                return PreInitState {
                    do_cleanup: false,
                    current_out_mode: 0,
                    current_in_mode: 0,
                };
            }

            let new_out_mode = current_out_mode | ENABLE_PROCESSED_OUTPUT |
                               ENABLE_WRAP_AT_EOL_OUTPUT |
                               ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            // ignore failure here and hope we are in a capable third party console
            SetConsoleMode(stdout_handle, new_out_mode);

            // TODO: ENABLE_MOUSE_INPUT and ENABLE_QUICK_EDIT_MODE need to be changed in
            // the mouse mode trait, not here for everything

            // TODO: it seems like ENABLE_VIRTUAL_TERMINAL_INPUT causes ^C to be passed
            // through in the input stream, overiding ENABLE_PROCESSED_INPUT. 
            // ENABLE_VIRTUAL_TERMINAL_INPUT is only used for mouse event handling at this
            // point. I'm not sure what the desired behaviour is but maybe it would be simpler
            // to start a thread and wait for the mouse events using the windows console
            // api and post them back in a similar fashion to the async reader.

            let new_in_mode = current_in_mode | ENABLE_VIRTUAL_TERMINAL_INPUT |
                              ENABLE_PROCESSED_INPUT |
                              ENABLE_MOUSE_INPUT;
            let new_in_mode = new_in_mode & !ENABLE_QUICK_EDIT_MODE;

            // ignore failure here and hope we are in a capable third party console
            SetConsoleMode(stdin_handle, new_in_mode);

            PreInitState {
                do_cleanup: true,
                current_out_mode,
                current_in_mode,
            }
        }
    }

    pub fn set_raw_input_mode(enable: bool) -> bool {
        unsafe {
            // TODO: this can fail -> INVALID_HANDLE_VALUE, GetLastError()
            let stdin_handle = GetStdHandle(STD_INPUT_HANDLE);

            let mut current_mode: DWORD = 0;
            if GetConsoleMode(stdin_handle, &mut current_mode) == FALSE {
                return false;
            }

            let new_mode = if enable {
                current_mode & !ENABLE_LINE_INPUT
            } else {
                current_mode | ENABLE_LINE_INPUT
            };

            SetConsoleMode(stdin_handle, new_mode) == TRUE
        }
    }

    // TODO: provide an implementation of this, perhaps just delegating to the atty crate?
    pub fn is_tty(_: &AsRawHandle) -> bool {
        true
    }

    /// Get the TTY device.
    ///
    /// This allows for getting stdio representing _only_ the TTY, and not other streams.
    #[cfg(target_os = "windows")]
    pub fn get_tty() -> io::Result<Box<io::Read>> {

        // TODO:
        // should this be CreateFile CONOUT$ ??

        // alternatively, return stdin if is_tty(stdin) else Err() ??

        // use std::env;
        // let tty = try!(env::var("TTY").map_err(|x| io::Error::new(io::ErrorKind::NotFound, x)));
        // fs::OpenOptions::new().read(true).write(true).open(tty)

        Ok(Box::new(io::stdin()))
    }
}

#[cfg(windows)]
pub use self::windows::{init, is_tty, get_tty, set_raw_input_mode, PreInitState};

