use proconio::input;
fn main() {
    input! {
        n: f32,
    }
    let (mut l, mut r) = (0.0, 100000.0);
    while (r - l) > 0.001 {
        let m: f32 = (l + r) / 2.0;
        let sum = m*m*m + m;
        if sum  >= n {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{l}")
}
