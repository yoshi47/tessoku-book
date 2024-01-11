use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }
    let mut dp = vec![0_isize; n];
    dp[0] = 0;
    dp[1] = (h[0] - h[1]).abs();
    for i in 2..n {
        dp[i] = cmp::min(dp[i-2] + (h[i-2]-h[i]).abs(), dp[i-1] + (h[i-1]-h[i]).abs())
    }
    println!("{}", dp[n-1]);
}
