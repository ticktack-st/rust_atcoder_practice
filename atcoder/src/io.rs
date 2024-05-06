use std::io;

pub fn read_textln() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("should String");
    buffer.trim().to_string()
}
