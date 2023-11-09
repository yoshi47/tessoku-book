use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    // let mut ans = 0;
    // let (mut l, mut r) = (0, n);
    // while l <= r {
    //     let m = (l + r) / 2;
    //     if x < a[m] {
    //         r = m -1;
    //     }
    //     if x == a[m] {
    //         ans = m + 1;
    //         break
    //     }
    //     if x > a[m] {
    //         l = m + 1;
    //     }
    // }
    let ans = a.binary_search(&x).unwrap() + 1;
    println!("{ans}");
}
