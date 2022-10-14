#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};
use std::{process::exit, collections::VecDeque};

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

#[derive(Debug)]
struct Pair(i32, i32);

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = scan.next::<i32>();
    let k = scan.next::<i32>();

    let mut queue = VecDeque::<Pair>::new();
    let mut visited = vec![false; 100002];
    queue.push_back(Pair(n as i32, 0));

    while !queue.is_empty() {
        let pair = queue.pop_front().unwrap();
        let pos = pair.0;
        let dist = pair.1;

        if pos == k {
            println!("{}", dist);
            exit(0);
        }

        if visited[pos as usize] {
            continue;
        }
        visited[pos as usize] = true;

        if pos * 2 <= 100001 && !visited[(pos * 2) as usize] {
            queue.push_front(Pair(pos*2, dist));
        }

        if pos + 1 <= 100001 && !visited[(pos + 1) as usize] {
            queue.push_back(Pair(pos+1, dist+1));
        }
        if pos - 1 >= 0 && !visited[(pos - 1) as usize] {
            queue.push_back(Pair(pos-1, dist+1));
        }
    }
}