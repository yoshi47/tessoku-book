use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", if (a..=b).any(|x| 100 % x == 0) {"Yes"} else {"No"});
}
