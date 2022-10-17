#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::{process::exit, collections::VecDeque, cmp::min};

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

    let n = scan.next::<i64>();
    let mut cnt = 0;
    let mut agent: (i64, i64) = (0, 0);
    let mut enemies = Vec::<(i64, i64)>::new();

    for i in 0..n {
        for j in 0..n {
            let x = scan.next::<i64>();
            match x {
                1 => {
                    enemies.push((i, j));
                    cnt += 1;
                },
                2 => {
                    agent = (i, j);
                },
                _ => {},
            }
        }
    }

    for e in enemies {
        let a = (e.0 - agent.0).abs() + (e.1 - agent.1).abs();
        if a % 2 == 1 {
            cnt -= 1;
        }
    }

    if cnt == 0 {
        println!("Lena");
    } else {
        println!("Kiriya");
    }
}