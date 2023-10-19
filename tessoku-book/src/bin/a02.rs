use proconio::input;
fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32; n],
    }
    let ans:bool = a.iter().any(|&aa| aa == x);
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }

    // println!("{}", if a.contains(&x) { "Yes" } else { "No" });
}
