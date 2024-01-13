use proconio::input;
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![true; 10001]; n+1];

    dp[0][0] = true;
    for i in 1..s+1 {
        dp[0][i] = false;
    }

    for i in 1..n+1 {
        for j in 0..s+1 {
            if j < a[i-1] {
                if dp[i-1][j] == true {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
            if j >= a[i-1] {
                if dp[i-1][j] == true || dp[i-1][j-a[i-1]] == true {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
        }
    }

    if dp[n][s] == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
