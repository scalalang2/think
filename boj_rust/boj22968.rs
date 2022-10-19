#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::{process::exit, collections::{VecDeque, HashMap}, cmp::min};

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

    let t = scan.next::<usize>();
    let max:u64 = 1000000001;
    let mut k:i64 = 2;
    let mut dp:Vec<u64> = Vec::new();
    dp.push(0);
    dp.push(1);

    while dp.last().unwrap().clone() < max {
        dp.push(dp[(k-2) as usize] + dp[(k-1) as usize] + 1);
        k += 1;
    }

    // println!("{:?}", dp);

    for _ in 0..t {
        let num = scan.next::<u64>();
        let mut ans = 0;

        for i in 0..dp.len() {
            if dp[i] > num {
                break;
            }
            ans = (i as i64);
        }

        println!("{}", ans);
    }
}