use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut dp = vec![vec![0_usize; t.len()+1]; s.len()+1];

    dp[0][0] = 0;
    for i in 0..s.len()+1 {
        for j in 0..t.len()+1 {
            if i >= 1 && j >= 1 && s[i-1] == t[j-1] {
                dp[i][j] = *[dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]].iter().min().unwrap();
            } else if i >= 1 && j >= 1 {
                dp[i][j] = *[dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+1].iter().min().unwrap();
            } else if i >= 1 {
                dp[i][j] = dp[i-1][j] + 1;
            } else if j >= 1 {
                dp[i][j] = dp[i][j-1] + 1;
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
