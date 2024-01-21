use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    }
    let (p, a): (Vec<_>, Vec<_>) = pa.into_iter().unzip();

    let mut dp = vec![vec![0; n];n];
    dp[0][n-1] = 0;

    for len in (0..n-1).rev() {
        for l in 0..n-len {
            let r = l + len;

            let mut score1 = 0;
            if l>=1 && l+1 <= p[l-1] && p[l-1] <= r+1 {
                score1 = a[l-1];
            }

            let mut score2 = 0;
            if r<=n-2 && l+1 <= p[r+1] && p[r+1] <= r+1 {
                score2 = a[r+1];
            }

            if l == 0 {
                dp[l][r] = dp[l][r+1] + score2;
            } else if r == n-1 {
                dp[l][r] = dp[l-1][r] + score1;
            } else {
                dp[l][r] = *[dp[l-1][r] + score1, dp[l][r+1] + score2].iter().max().unwrap();
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(dp[i][i]);
    }
    println!("{ans}");
}