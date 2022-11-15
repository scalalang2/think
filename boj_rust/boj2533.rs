#[allow(unused_imports)]
use std::io::*;
use std::*;

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
    
    let n = scan.next::<usize>();
    let mut graph:Vec<Vec<usize>> = (0..=n).map(|_| Vec::<usize>::new()).collect();
    let mut tree:Vec<Vec<usize>> = (0..=n).map(|_| Vec::<usize>::new()).collect();
    let mut visited:Vec<bool> = vec![false; n+1];
    let mut dp = vec![vec![-1; 2]; n+1];

    for _ in 0..(n-1) {
        let u = scan.next::<usize>();
        let v = scan.next::<usize>();
        graph[u].push(v);
        graph[v].push(u);
    }

    fn make_tree(graph:&Vec<Vec<usize>>, tree:&mut Vec<Vec<usize>>, visited: &mut Vec<bool>, node:usize) {
        visited[node] = true;
        for &v in graph[node].iter() {
            if !visited[v] {
                tree[node].push(v);
                make_tree(graph, tree, visited, v);
            }
        }
    }

    make_tree(&graph, &mut tree, &mut visited, 1);

    fn f(k: usize, pe: bool, dp: &mut Vec<Vec<i32>>, graph: &mut Vec<Vec<usize>>) -> i32 {
        if dp[k][pe as usize] != -1 {
            return dp[k][pe as usize];
        }

        let mut picked = 1;
        let mut not_pick = 1 << 30;

        if pe {
            not_pick = 0;
        }

        for v in 0..graph[k].len() {
            picked += f(graph[k][v], true, dp, graph);
            if pe {
                not_pick += f(graph[k][v], false, dp, graph);
            }
        }

        dp[k][pe as usize] = cmp::min(picked, not_pick);
        return dp[k][pe as usize];
    }

    writeln!(out, "{}", f(1, true, &mut dp, &mut tree)).unwrap();
}
