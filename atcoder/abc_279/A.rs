#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
 
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
 
    let mut ans = 0;
    let text = scan.next::<String>();
    text.chars().for_each(|c| {
        if c == 'v' {
            ans += 1;
        } else {
            ans += 2;
        }
    });
 
    writeln!(out, "{}", ans).ok();
}