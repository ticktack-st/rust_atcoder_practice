mod reader;

fn main() {
    let buf = reader::read_vec_char();
    println!("{:?}", buf);
}
