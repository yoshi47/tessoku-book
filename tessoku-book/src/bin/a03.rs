use proconio::input;
fn main() {
    input! {
        n: i32,
        k: i32,
        p: [i32; n],
        q: [i32; n],
    }
    for pp in &p {
        for qq in &q {
            if pp + qq == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No")
}
