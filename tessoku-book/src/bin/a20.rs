use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let (ss, tt) = (s.len(), t.len());
    let mut dp = vec![vec![0_usize; tt+1]; ss+1];
    dp[0][0] = 0;
    for i in 0..ss+1 {
        for j in 0.. tt+1 {
            if i >= 1 && j >= 1 && s[i-1] == t[j-1] {
                dp[i][j] = *[dp[i-1][j], dp[i][j-1], dp[i-1][j-1]+1].iter().max().unwrap();
            }
            else if i >= 1 && j >= 1 {
                dp[i][j] = *[dp[i-1][j], dp[i][j-1]].iter().max().unwrap();
            }
            else if i >= 1 {
                dp[i][j] = dp[i-1][j];
            }
            else if j >= 1 {
                dp[i][j] = dp[i][j-1];
            }
        }
    }
    println!("{}", dp[ss][tt]);
}
