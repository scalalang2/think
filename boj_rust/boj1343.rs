#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::{process::exit, collections::{VecDeque, HashMap}, cmp::min};

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
    let mut answer = String::new();
    let text = scan.next::<String>();
    let length = text.len();
    
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    loop {
        if i >= length {
            break; 
        }

        if chars[i] == '.' {
            answer += ".";
            i += 1;
            continue;
        }

        if i + 3 < length && chars[i] == 'X' && chars[i+1] == 'X' && chars[i+2] == 'X' && chars[i+3] == 'X' {
            answer += "AAAA";
            i += 4;
            continue;
        }

        if i + 1 < length && chars[i] == 'X' && chars[i+1] == 'X' {
            answer += "BB";
            i += 2;
            continue;
        }

        println!("{}", -1);
        exit(0);
    }

    println!("{}", answer);
}