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

    let (n, t):(usize, u64) = (scan.next(), scan.next());
    let playlist:Vec<u64> = (0..n).map(|_| scan.next()).collect();
    let sum = playlist.iter().sum::<u64>();
    let mut time = (t) % sum;

    for i in 0..n {
        if time <= playlist[i] {
            println!("{} {}", i+1, time);
            exit(0);
        }
        time -= playlist[i];
    }
    
}