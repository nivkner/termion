extern crate  kernel32;

use std::process::Command;
use std::env;

fn main() {

    let allocated = unsafe { kernel32::AllocConsole() != 0 };
    println!("AllocConsole allocated console: {}", allocated);

    let mut process_args : Vec<String> = env::args().collect();

    let status = Command::new(&process_args[1])
                     .args(&process_args[2..])
                     .status()
                     .expect("failed to execute process");
    println!("exit status was {}", status);
}
