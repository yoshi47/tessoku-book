use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    let ans = usize::from_str_radix(&n.to_string(), 2);
    match ans {
        Ok(x) => println!("{}", x),
        Err(_e) => return,
    }
}
