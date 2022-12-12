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

    let mut n: i64 = scan.next();
    for i in (0..=n).rev() {
        println!("{}", i); 
    }
}