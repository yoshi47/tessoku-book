use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut asum = vec![0_usize; n+1];
    for i in 0..n {
        asum[i+1] = asum[i] + a[i];
    }
    let mut ans = 0;
    let mut r = 0;
    for l in 1..n+1 {
        while r < n+1 && asum[r] - asum[l-1] <= k {
            r += 1;
        }
        ans += r - l;
    }

    println!("{ans}");
}
