use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }

    let mut dp = vec![isize::MIN; n+1];

    dp[1] = 0;
    for i in 1..n {
        dp[a[i-1]] = dp[a[i-1]].max(dp[i]+100);
        dp[b[i-1]] = dp[b[i-1]].max(dp[i]+150);
    }

    println!("{}", dp[n]);
}
