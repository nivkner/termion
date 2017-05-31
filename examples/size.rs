extern crate termion;

use termion::terminal_size;

fn main() {
    let named = termion::init();

    println!("Size is {:?}", terminal_size().unwrap())
}
