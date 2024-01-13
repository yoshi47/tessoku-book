use proconio::input;
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; 10001]; n+1];

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

    if dp[n][s] == false {
        println!("-1");
        return
    }

    let mut ans = Vec::new();
    let mut place = s;
    for i in (0..n+1).rev() {
        if dp[i][place] == true {
            place = place - 0
        } else {
            place -= a[i];
            ans.push(i+1);
        }
    }
    println!("{}", ans.len());
    for i in (0..ans.len()).rev() {
        print!("{} ", ans[i]);
    }
}

// dp[n][s]