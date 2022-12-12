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
 
fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
 
    let s = scan.next::<String>();
    let t = scan.next::<String>();
 
    for i in 0..s.len() {
        if s[i..].starts_with(&t) {
            println!("Yes");
            exit(0);
        }
    }
 
    println!("No");
}