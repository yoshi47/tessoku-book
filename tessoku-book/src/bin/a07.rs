use proconio::input;
fn main() {
    input! {
        d: usize,
        p: [(usize, usize)],
    }
    let mut b = vec![0_i32; d+1];
    for (l, r) in p {
        b[l-1] += 1;
        b[r] -= 1;
    }

    let mut ans = 0;
    for i in 0..d {
        ans += b[i];
        println!("{}", ans);
    }
}
