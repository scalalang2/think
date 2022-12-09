#[allow(unused_imports)]
use std::io::*;
use std::{*, cmp::Ordering, collections::{HashMap, VecDeque, HashSet}};

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

    let (n, m): (usize, usize) = (scan.next(), scan.next());
    let mut arr:Vec<u64> = vec![0; n];
    let mut max_seg_tree:Vec<u64> = vec![];
    let mut min_seg_tree:Vec<u64> = vec![];

    fn construct(tree: &mut Vec<u64>, n: usize) -> usize {
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

    fn update_max(tree: &mut Vec<u64>, index: usize, size: usize, val: u64) {
        let mut i = index + size / 2;
        tree[i] = val;
        while i > 1 {
            i /= 2;
            tree[i] = tree[i * 2].max(tree[i * 2 + 1]);
        }
    }

    fn update_min(tree: &mut Vec<u64>, index: usize, size: usize, val: u64) {
        let mut i = index + size / 2;
        tree[i] = val;
        while i > 1 {
            i /= 2;
            tree[i] = tree[i * 2].min(tree[i * 2 + 1]);
        }
    }

    fn find_max(tree: &mut Vec<u64>, pos: usize, low: usize, high: usize, left: usize, right: usize) -> u64 {
        if left > high || right < low {
            return 0;
        }
        
        if left <= low && right >= high {
            return tree[pos];
        }

        let mid = (low + high) / 2;
        find_max(tree, pos * 2, low, mid, left, right).max(find_max(tree, pos * 2 + 1, mid + 1, high, left, right))
    }

    fn find_min(tree: &mut Vec<u64>, pos: usize, low: usize, high: usize, left: usize, right: usize) -> u64 {
        if left > high || right < low {
            return u64::MAX;
        }

        if left <= low && right >= high {
            return tree[pos];
        }

        let mid = (low + high) / 2;
        find_min(tree, pos * 2, low, mid, left, right).min(find_min(tree, pos * 2 + 1, mid + 1, high, left, right))
    }

    construct(&mut max_seg_tree, n);
    let size = construct(&mut min_seg_tree, n);
    for i in 0..n {
        let v:u64 = scan.next();
        update_max(&mut max_seg_tree, i, size, v);
        update_min(&mut min_seg_tree, i, size, v);
    }

    for _ in 0..m {
        let (l, r): (usize, usize) = (scan.next(), scan.next());
        let max = find_max(&mut max_seg_tree, 1, 1, size / 2, l,  r);
        let min = find_min(&mut min_seg_tree, 1, 1, size / 2, l, r);
        println!("{} {}", min, max);
    }

    
    // 생각 해볼 거리, segtree 에서는 무조건 루트 부터 시작해서 값을 찾기 때문에 pos는 무조건 1로 시작한다.
    // 구간을 찾는 것기 때문에, left, right는 내가 찾을 구간을 나타내고
    // low, high는 현재 구간을 나타낸다.
    // low, high는 항상 트리의 모든 구간을 포함한다.

    // l, r은 실제 트리에서 인덱스를 찾는 거랑은 상관이 없구나
    // 그냥 구간이 포함되는지, 안되는지만 찾고 실제 트리에서 값을 가져오는 애는 pos 값이기 때문에
    // l,r 은 구간에 포함되도록 -1 씩 빼주는게 맞다. 배열 처럼
}