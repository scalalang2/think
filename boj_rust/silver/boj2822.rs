#[allow(unused_imports)]
use std::io::*;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    *, rc::Rc,
};

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

    let n: usize = scan.next();
    let m: usize = scan.next();

    let mut dp = vec![0; n+1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i-1] + dp[i-2];
    }

    let mut tmp = 0;
    let mut ans = 1;
    for _ in 0..m {
        let x: usize = scan.next();
        ans = ans * dp[x-tmp-1];
        tmp = x;
    }

    ans = ans * dp[n-tmp];

    println!("{}", ans);

}
