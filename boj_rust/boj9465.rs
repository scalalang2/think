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

const MAX_SIZE: usize = 100000;

fn solve(c: usize, status: usize, n: usize, value: &mut Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if c == n {
        return 0;
    }

    if dp[status][c] != -1 {
        return dp[status][c];
    }

    let mut result = solve(c+1, 0, n, value, dp);
    if status != 1 {
        result = cmp::max(result, solve(c+1, 1, n, value, dp) + value[0][c]);
    }
    if status != 2 {
        result = cmp::max(result, solve(c+1, 2, n, value, dp) + value[1][c]);
    }

    dp[status][c] = result;
    result
}

fn main() {
    let mut scan = Scanner::default();
    let mut out = BufWriter::new(stdout());
    let t = scan.next::<usize>();
    
    for _ in 0..t {
        let n = scan.next::<usize>();
        let mut value:Vec<Vec<i32>> = (0..2).map(|_| vec![0; MAX_SIZE+1]).collect();
        let mut dp:Vec<Vec<i32>> = (0..3).map(|_| vec![-1; MAX_SIZE+1]).collect();

        for i in 0..2 {
            for j in 0..n {
                value[i][j] = scan.next::<i32>();
            }
        }

        writeln!(out, "{}", solve(0, 0, n, &mut value, &mut dp)).ok();
    }
}
