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

    let t = scan.next::<usize>();
    for _ in 0..t {
        let ll = scan.next::<usize>();

        let m1:HashMap<String, usize> = (0..ll)
            .map(|x| (scan.next::<String>(), x))
            .collect();

        let m2: HashMap<usize, usize> = (0..ll)
            .map(|x| {
                let tmp = scan.next::<String>();
                let tt = m1.get(&tmp).unwrap();
                (*tt, x)
            })
            .collect();
        
        let mut ans = String::new();
        let plain_text_arr: Vec<String> = (0..ll).map(|_| scan.next::<String>()).collect();
        for i in 0..ll {
            let target = m2.get(&i).unwrap();
            ans.push_str(&plain_text_arr[*target]);
            ans.push_str(" ");
        }

        println!("{}", ans);
    }
}
