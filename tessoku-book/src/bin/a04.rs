use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    let nlen = { format!("{:b}", 1000).to_string().len() };
    for i in (0..nlen).rev() {
        let wari: usize = 2usize.pow(i as u32);
        print!("{}", (n / wari) % 2);
    }
    println!();


    // println!("{:010b}", n);
}
