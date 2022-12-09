#[allow(unused_imports)]
use std::io::*;
use std::{*, cmp::Ordering, collections::HashMap};

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
    macro_rules! print { ($($tt:tt)*) => { write!(out, $($tt)*).unwrap() }; }
    macro_rules! println { ($($tt:tt)*) => { writeln!(out, $($tt)*).unwrap() }; }

    let n:usize = scan.next();
    let mut arr:Vec<(i64, i64)> = vec![(0, 0);n];
    let mut tree:Vec<i64> = vec![];

    fn make_tree(tree: &mut Vec<i64>, n:usize) -> usize {
        let mut size = 1;
        loop {
            size <<= 1;
            if size > n {
                break;
            }
        }

        size *= 2;
        tree.resize(size, 0);
        size
    }

    fn update(tree: &mut Vec<i64>, pos: usize, size: usize, value: i64) {
        let mut pos = pos + size / 2;
        tree[pos] = value;

        while pos > 1 {
            pos /= 2;
            tree[pos] = tree[pos * 2].max(tree[pos * 2 + 1]);
        }
    }

    fn max_tree(tree: &mut Vec<i64>, pos: usize, low: usize, high: usize, left: usize, right: usize) -> i64 {
        if left > high || right < low {
            return 0;
        }

        if left <= low && right >= high {
            return tree[pos];
        }

        let mid = (low + high) / 2;
        max_tree(tree, pos * 2, low, mid, left, right).max(max_tree(tree, pos * 2 + 1, mid + 1, high, left, right))
    }

    let size = make_tree(&mut tree, n);
    for i in 0..n {
        let value: i64 = scan.next();
        arr[i] = (value, i as i64);
    }
    arr.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Ordering::Equal => a.1.cmp(&b.1).reverse(),
            other => other,
        }
    });

    let mut ans: i64 = 0;
    let mut max_cnt:i64 = 0;
    for i in 0..n {
        let (_, index) = arr[i];

        // 2배씩 늘어나는 것 2^k 이기 때문에 리프노드의 개수는
        // size / 2 이지만, 인덱스를 고려하면 0~size/2-1 이다.
        let max = max_tree(&mut tree, 1, 0, size / 2 - 1, 0, index as usize);

        // 각 오퍼레이션 마다 나타난 최댓값을 정답으로 업데이트 한다.
        if ans < max + 1 {
            ans = max + 1;
            max_cnt = 1;
        } else if ans == max + 1 {
            max_cnt += 1;
            max_cnt %= 1000000007;
        }
        update(&mut tree, index as usize, size, max + 1);
    }

    println!("{} {}", ans, max_cnt);
}