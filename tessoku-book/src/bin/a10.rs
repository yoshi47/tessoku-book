use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [(usize, usize)],
    }
    let mut p = vec![0_usize; n];
    let mut q = vec![0_usize; n];

    p[0] += a[0];
    for i in 1..n {
        p[i] = p[i-1].max(a[i]);
    }

    q[n-1] += a[n-1];
    for i in (0..n-1).rev() {
        q[i] = q[i+1].max(a[i]);
    }

    for (l, r) in b {
        let ans = std::cmp::max(p[l-2], q[r]);
        println!("{}", ans);
    }

}
