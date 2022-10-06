#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::process::exit;

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

fn dc(n: i32, k: i32, l: i32) {
    let ll = l*2 + k+3;
    if n <= 3 {
        if n == 1 {
            println!("m");
        } else {
            println!("o");
        }
        exit(0);
    }

    if ll < n {
        dc(n, k+1, ll);
    } else {
        if n > l && n <= l+k+3 {
            if n-l != 1 {
                println!("o");
            } else {
                println!("m");
            }
            exit(0);
        } else {
            dc(n-l-k-3, 1, 3);
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = scan.next::<usize>();
    dc(n as i32, 1, 3);
}