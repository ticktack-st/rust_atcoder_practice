mod io;

fn main() {
    let buf = io::read_vec_char();
    println!("{:?}", buf);
}
