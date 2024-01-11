use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut t = a.to_vec();
    t.sort();
    t.dedup();

    let mut b = vec![0_usize; n];
    for i in 0..n {
        b[i] = t.binary_search(&a[i]).unwrap() + 1;
    }
    let bb: Vec<String> = b.iter().map(|&x| x.to_string()).collect();
    println!("{}", bb.join(" "));
}
