#[allow(unused_imports)]
use std::io::*;
use std::*;

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
    
    let len = scan.next::<usize>();
    let k = scan.next::<usize>();
    let x = (0..k).map(|_| scan.next::<i64>()).collect::<Vec<_>>();
    let mut ans = 0;
    ans = cmp::max(x[0], (len as i64) - x[k-1]);
    for i in 1..k {
        ans = cmp::max(ans, ((x[i] - x[i-1]) as f64 / 2.0_f64).ceil() as i64);
    }

    writeln!(out, "{}", ans).unwrap();
}
