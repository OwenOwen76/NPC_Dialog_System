use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn typewriter_print(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(40));
    }
}

pub fn typewriter_println(text: &str) {
    typewriter_print(text);
    println!();
}
