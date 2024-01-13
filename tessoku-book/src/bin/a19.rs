use proconio::input;
use std::cmp;
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![isize::MIN; 100001]; n+1];

    dp[0][0] = 0;

    for i in 1..n+1 {
        let (w1, v1) = wv[i-1];
        for j in 0..w+1 {
            if j < w1 {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = cmp::max(dp[i-1][j], dp[i-1][j-w1] + v1 as isize);
            }
        }
    }
    let mut ans = 0;
    for i in 0..w+1 {
        ans = ans.max(dp[n][i]);
    }
    println!("{ans}");
}
