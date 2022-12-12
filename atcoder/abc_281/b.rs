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

    let mut txt:String = scan.next();
    let ch = txt.chars().collect::<Vec<_>>();

    let mut ans = "Yes";
    let mut i = 0;

    if txt.len() != 8 {
        ans = "No";
        println!("{}", ans);
        return;
    }

    if ch[0] < 'A' || ch[0] > 'Z' {
        ans = "No";
        println!("{}", ans);
        return;
    }

    if ch[7] < 'A' || ch[7] > 'Z' {
        ans = "No";
        println!("{}", ans);
        return;
    }

    let num = ch[1..7].iter().fold("".to_string(), |acc, x| acc.to_string() + &x.to_string()).parse::<i32>();
    if num.is_err() {
        ans = "No";
        println!("{}", ans);
        return;
    }

    let num = num.unwrap();
    if num < 100000 || num > 999999 {
        ans = "No";
        println!("{}", ans);
        return;
    }

    println!("{}", ans);
    
}