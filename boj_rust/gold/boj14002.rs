#[allow(unused_imports)]
use std::io::*;
use std::{*, cmp::Ordering, collections::HashMap};

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
    let mut out = BufWriter::new(stdout());
    macro_rules! print { ($($tt:tt)*) => { write!(out, $($tt)*).unwrap() }; }
    macro_rules! println { ($($tt:tt)*) => { writeln!(out, $($tt)*).unwrap() }; }

    let n:usize = scan.next();
    let a:Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let mut dp:Vec<i64> = vec![1; n];
    for i in 1..n {
        for j in 0..i {
            if a[i] > a[j] {
                dp[i] = cmp::max(dp[i], dp[j] + 1);
            }
        }
    }

    let mut v = dp.iter().max().unwrap().clone();
    let mut ans:Vec<i64> = vec![];
    for i in (0..n).rev() {
        if dp[i] == v {
            ans.push(a[i]);
            v -= 1;
        }
    }

    ans.reverse();
    println!("{}", dp.iter().max().unwrap());
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}