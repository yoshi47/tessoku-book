use proconio::input;
fn main() {
    input! {
        mut a: [usize],
        x: [usize],
    }
    a.sort();
    for i in x {
        match a.binary_search(&i) {
            Ok(ans) => println!("{ans}"),
            Err(ans) => println!("{ans}"),
        }
    }
}
