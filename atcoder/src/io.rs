use std::io;

pub fn read_textln() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("should String");
    buffer.trim().to_string()
}

pub fn read_u32() -> u32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse::<u32>().expect("should number"),
        Err(e) => panic!("error: {}", e),
    }
}

pub fn read_i32() -> i32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse::<i32>().expect("should number"),
        Err(e) => panic!("error: {}", e),
    }
}
