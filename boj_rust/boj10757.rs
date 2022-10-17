#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::{process::exit, collections::VecDeque, cmp::min};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut a = vec![0; 10002];
    let mut b = vec![0; 10002];
    let mut ans = "".to_string();

    let a_str = scan.next::<String>();
    let b_str = scan.next::<String>();

    for (i, c) in a_str.chars().rev().enumerate() {
        a[i] = c.to_digit(10).unwrap();
    }

    for (i,c) in b_str.chars().rev().enumerate() {
        b[i] = c.to_digit(10).unwrap();
    }

    for i in 0..10001 {
        let mut sum = a[i] + b[i];
        if sum >= 10 {
            a[i + 1] += 1;
            sum -= 10;
        }
        ans = format!("{}{}", ans, sum);
    }

    let ans = ans.trim_end_matches("0").chars().rev().collect::<String>();
    println!("{}", ans);

}