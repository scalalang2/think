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

fn find(parent: &mut Vec<i64>, dist: &mut Vec<i64>, i: i64) -> i64 {
    let x = i as usize;
    if parent[x] == i {
        return i;
    }

    // 경로 압축
    let p = find(parent, dist, parent[x]);
    dist[x] += dist[parent[x] as usize];
    parent[x] = p;
    
    return p;
}

fn union(parent: &mut Vec<i64>, dist: &mut Vec<i64>, x: i64, y: i64) {
    // let x_parent = find(parent, dist, x);
    // let y_parent = find(parent, dist, y);
    parent[x as usize] = y;
    dist[x as usize] = (x - y).abs() % 1000;
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    // T : number of test case
    let t = scan.next::<usize>();

    for _ in 0..t {
        // N : number of companies
        let n = scan.next::<usize>();

        let parent = &mut vec![0; n+1];
        let dist = &mut vec![0; n+1];

        for i in 0..n+1 {
            parent[i] = i as i64;
        }

        loop {
            let cmd = scan.next::<String>();
            match cmd.as_str() {
                "E" => {
                    let e = scan.next::<i64>();
                    let _ = find(parent, dist, e);
                    writeln!(out, "{}", dist[e as usize]).ok();
                },
                "I" => {
                    let v = scan.next::<i64>();
                    let u = scan.next::<i64>();
                    union(parent, dist, v, u);
                },
                "O" => {
                    break;
                },
                _ => panic!("Unknown command")
            }
        }
    }
}