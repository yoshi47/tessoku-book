use proconio::input;
fn main() {
    input! {
        n: usize,
        w: isize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![isize::MAX; 100_009]; n+1];
    dp[0][0] = 0;
    for i in 1..n+1 {
        let (ww, vv) = wv[i-1];
        for j in 0..100_001 {
            dp[i][j] = dp[i-1][j];
            if j >= vv && !(dp[i-1][j-vv] == isize::MAX){
                dp[i][j] = dp[i][j].min(dp[i-1][j-vv] + ww as isize);
            }
        }
    }
    let mut ans = 0;
    for i in 0..100_001 {
        if dp[n][i] <= w {
            ans = i;
        }
    }
    println!("{ans}");
}
