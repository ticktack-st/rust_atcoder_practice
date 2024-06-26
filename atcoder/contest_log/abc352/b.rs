use std::io;

pub fn read_vec_char() -> Vec<char> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("should string");
    buffer.trim().chars().collect()
}

fn calc(s: Vec<char>, t: Vec<char>) -> Vec<usize> {
    let mut ans = Vec::<usize>::new();
    let mut j = 0;
    for i in 0..s.len() {
        while j < t.len() {
            if s[i] == t[j] {
                ans.push(j + 1);
                j += 1;
                break;
            } else {
                j += 1;
            }
        }
    }
    ans
}

fn main() {
    let s = read_vec_char();
    let t = read_vec_char();

    let ans = calc(s, t);
    for i in ans {
        print!("{} ", i);
    }
    println!("");
}
