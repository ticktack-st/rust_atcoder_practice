use std::io;

fn main() {
    let nxyz = read_vec_i32();
    if nxyz[1] < nxyz[3] && nxyz[3] < nxyz[2] {
        println!("Yes");
    } else if nxyz[2] < nxyz[3] && nxyz[3] < nxyz[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn read_vec_i32() -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("should string array");
    buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("should number"))
        .collect()
}
