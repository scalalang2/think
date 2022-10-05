#[allow(unused_imports)]
use std::io::{stdout, BufWriter, Stdout, Write};

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

fn dfs(
    v: usize,
    graph: &Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    cnt: &mut i32,
    cnt_table: &mut Vec<i32>,
) {
    visited[v] = true;
    cnt_table[v] = *cnt;
    *cnt += 1;

    for &next in graph[v].iter() {
        if !visited[next as usize] {
            dfs(next as usize, graph, visited, cnt, cnt_table);
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let v = scan.next::<usize>();
    let e = scan.next::<usize>();
    let s = scan.next::<usize>();
    let mut cnt = 1;

    let mut cnt_table = (0..v+1).map(|_| 0).collect::<Vec<_>>();
    let mut visited = (0..v+1).map(|_| false).collect::<Vec<_>>();
    let mut graph = (0..v+1).map(|_| Vec::<i32>::new()).collect::<Vec<_>>();

    for _ in 0..e {
        let a = scan.next::<i32>();
        let b = scan.next::<i32>();
        graph[a as usize].push(b);
        graph[b as usize].push(a);
    }

    for i in 0..v+1 {
        graph[i].sort();
    }

    dfs(s, &graph, &mut visited, &mut cnt, &mut cnt_table);

    for i in 1..v+1 {
        writeln!(out, "{}", cnt_table[i]).ok();
    }
}


