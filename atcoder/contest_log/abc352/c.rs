use std::io;

fn read_u32() -> u32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse::<u32>().expect("should number"),
        Err(e) => panic!("error: {}", e),
    }
}

fn read_vec_u32() -> Vec<u32> {
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

fn read_vec_vec(n: u32) -> Vec<Vec<u32>> {
    let mut ab: Vec<Vec<u32>> = vec![];
    for _i in 0..n {
        ab.push(read_vec_u32());
    }
    ab
}

fn calc(ab: Vec<Vec<u32>>) -> usize {
    let sum: usize = ab.iter().map(|x| usize::try_from(x[0]).unwrap()).sum();
    let mut max = &sum - usize::try_from(ab[0][0]).unwrap() + usize::try_from(ab[0][1]).unwrap();

    for i in 0..ab.len() {
        let tmp = &sum - usize::try_from(ab[i][0]).unwrap() + usize::try_from(ab[i][1]).unwrap();
        if max < tmp {
            max = tmp;
        }
    }
    max
}

fn main() {
    let n = read_u32();
    let ab = read_vec_vec(n);

    let ans = calc(ab);
    println!("{}", ans);
}
