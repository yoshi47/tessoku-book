use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        qq: [[i32;2]; q],
    }

    let mut s = vec![0_i32; n+1];
    for i in 1..n+1 {
        s[i] = s[i-1] + a[i-1];
    }

    for i in 0..q {
        println!("{}", s[qq[i][1] as usize] - s[qq[i][0] as usize - 1]);
    }
}
