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

const MAX: i32 = 100002;

fn main() {
    let mut scan = Scanner::default();
    let n = scan.next::<usize>();
    let mut dp = vec![-1; MAX as usize];
    dp[2] = 1;
    dp[4] = 2;
    dp[5] = 1;

    for i in 6..MAX {
        if dp[(i-2) as usize] != -1 && dp[(i-5) as usize] != -1 {
            dp[i as usize] = min(dp[(i-2) as usize], dp[(i-5) as usize]) + 1;
        } else if dp[(i-2) as usize] != -1 {
            dp[i as usize] = dp[(i-2) as usize] + 1;
        } else if dp[(i-5) as usize] != -1 {
            dp[i as usize] = dp[(i-5) as usize] + 1;
        }
    }

    println!("{}", dp[n]);
}