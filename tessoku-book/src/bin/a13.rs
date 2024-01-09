use proconio::input;
fn main() {
    input! {
        n : usize,
        k: usize,
        a: [usize; n],
    }

    let mut r = vec![0_usize; n+1];
    for i in 0..n-1 {
        if i == 0 {
            r[i] = 1;
        } else {
            r[i] = r[i-1];
        }

        while r[i] < n && a[r[i]]-a[i] <= k {
            r[i] += 1;
        }
    }
    let mut ans = 0;
    for i in 0..n-1 {
        ans += r[i] - (i+1);
    }
    println!("{ans}");
}
