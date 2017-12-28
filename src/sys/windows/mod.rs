extern crate winapi;

use std::ops::{Deref, DerefMut};

pub mod tty;
pub mod size {
    use std::io;
    pub fn terminal_size() -> io::Result<(u16, u16)> {
        unimplemented!()
    }
}

pub mod attr {
    use super::Termios;
    use std::io;
    pub fn get_terminal_attr() -> io::Result<Termios> {
        unimplemented!()
    }
    pub fn set_terminal_attr(termios: &Termios) -> io::Result<()> {
        unimplemented!()
    }
    pub fn raw_terminal_attr(ios: &mut Termios) {
        unimplemented!()
    }
}

pub type tcflag_t = u32;
pub type cc_t = u8;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; 32],
}

impl Default for Termios {
    fn default() -> Termios {
        unimplemented!()
    }
}

impl Termios {
    pub fn make_raw(&mut self) {
        unimplemented!()
    }
}

impl Deref for Termios {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl DerefMut for Termios {
    fn deref_mut(&mut self) -> &mut [u8] {
        unimplemented!()
    }
}
