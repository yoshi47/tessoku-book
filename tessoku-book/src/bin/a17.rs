use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-2],
    }
    let mut dp = vec![0_usize; n];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = cmp::min(dp[i-1] + a[i-1], dp[i-2] + b[i-2]);
    }
    let mut ans = Vec::new();
    let mut place = n-1;
    loop {
        ans.push(place);
        if place == 0 {break};

        if dp[place-1] + a[place-1] == dp[place] {
            place -= 1;
        } else {
            place -= 2;
        }
    }
    println!("{}", ans.len());
    for i in ans.iter().rev() {
        print!("{} ", i+1);
    }
}
