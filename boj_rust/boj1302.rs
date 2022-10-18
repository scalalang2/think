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
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<i64>();
    let mut score: HashMap<String, i64> = HashMap::new();

    for _ in 0..n {
        let book = scan.next::<String>();
        match score.insert(book.clone(), 1) {
            Some(v) => {
                score.insert(book, v + 1);
            },
            None => {
                // nothing to do
            }
        }
    }

    let mut ans:String = "".to_string();
    let mut max:i64 = 0;

    let mut keys:Vec<&String> = score.keys().collect();
    keys.sort();

    for key in keys {
        let v = score.get(key).unwrap();
        if *v > max {
            max = *v;
            ans = key.clone();
        }
    }
    println!("{}", ans);
}