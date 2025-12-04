use std::io::{self, Write};

pub fn input(message: &str) -> String {
    print!("{}: ", message);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
