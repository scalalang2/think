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
 
    let h = scan.next::<usize>(); // height
    let w = scan.next::<usize>(); // width
 
    let mut cnt = vec![vec![0; h]; 2];
 
    for i in 0..2 {
        for j in 0..h {
            let line = scan.next::<String>();
            line.chars().for_each(|c| {
                if c == '#' {
                    cnt[i][j] += 1;
                }
            })
        }
    }
 
    for i in 0..h {
        if cnt[0][i] != cnt[1][i] {
            println!("No");
            exit(0);
        }
    }
 
    println!("Yes");
}