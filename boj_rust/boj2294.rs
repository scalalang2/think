#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::{process::exit, collections::{VecDeque, HashMap}, cmp::{min, self}};

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

// f(k) = min(f(k-n) + 1, ....)
const NOT_FOUND: i32 = 10001;
const NOT_EVALUATED: i32 = -1;

fn main() {
    let mut scan = Scanner::default();
    let mut out = BufWriter::new(stdout());
    let n = scan.next::<usize>();
    let k = scan.next::<usize>();
    let mut dp = vec![NOT_EVALUATED; 10001];
    let mut coins = vec![0; n];

    for i in 0..n {
        coins[i] = scan.next::<i32>();
    }

    fn f(k: i32, dp: &mut Vec<i32>, coins: &Vec<i32>) -> i32 {
        if k == 0 {
            return 0;
        }

        if dp[k as usize] != NOT_EVALUATED {
            return dp[k as usize];
        }

        let mut result = NOT_FOUND;
        for i in 0..coins.len() {
            if k-coins[i] >= 0 && f(k-coins[i], dp, coins) != NOT_FOUND {
                result = min(result, f(k-coins[i], dp, coins) + 1);
            }
        }

        dp[k as usize] = result;
        return result;
    }

    let answer = f(k as i32, &mut dp, &coins);
    if answer == NOT_FOUND {
        writeln!(out, "-1").ok();
    } else {
        writeln!(out, "{}", answer).ok();
    }
}
