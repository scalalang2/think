#[allow(unused_imports)]
use std::io::*;
use std::{*, cmp::Ordering};

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
    
   let diff = 2541 - 1998;
   let y = scan.next::<usize>();
   println!("{}", y - diff);

}
