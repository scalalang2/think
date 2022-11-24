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
    
    let S1 = scan.next::<String>().chars().collect::<Vec<char>>();
    let S2 = scan.next::<String>().chars().collect::<Vec<char>>();
    let mut dp = vec![vec![0 as i32; S2.len() + 1]; S1.len() + 1];

    for i in 0..S1.len() {
        for j in 0..S2.len() {
            if S1[i] == S2[j] {
                dp[i+1][j+1] = dp[i][j] + 1;
            } else {
                dp[i+1][j+1] = cmp::max(dp[i][j+1], dp[i+1][j]);
            }
        }
    }

    fn lcs(i: usize, j: usize, dp: &Vec<Vec<i32>>, S1:&Vec<char>, S2:&Vec<char>) -> String {
        if dp[i][j] == 0 {
            return String::new();
        }

        if S1[i-1] == S2[j-1] {
            return lcs(i-1, j-1, dp, S1, S2) + &S1[i-1].to_string();
        } else if dp[i-1][j] > dp[i][j-1] {
            return lcs(i-1, j, dp, S1, S2);
        } else {
            return lcs(i, j-1, dp, S1, S2);
        }
    }
    
    let ans = lcs(S1.len(), S2.len(), &dp, &S1, &S2);
    writeln!(out, "{}", dp[S1.len()][S2.len()]).ok();

    if dp[S1.len()][S2.len()] > 0 {
        writeln!(out, "{}", ans).ok();
    }
}
