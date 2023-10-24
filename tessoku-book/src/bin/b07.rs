use proconio::input;
fn main() {
    input! {
        t: usize,
        p: [(usize, usize)],
    }
    let mut b = vec![0; t+1];

    for (l, r) in p {
        b[l] += 1;
        b[r] -= 1;
    }

    let mut ans = 0;
    for i in 0..t {
        ans += b[i];
        println!("{ans}");
    }
}
