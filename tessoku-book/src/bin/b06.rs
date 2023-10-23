use proconio::input;
fn main() {
    input! {
        a: [usize],
        q: [(usize, usize)],
    }

    let mut s = vec![0_usize; a.len()+1];
    for i in 0..a.len() {
        s[i+1] = s[i] + a[i];
    }

    for (l, r) in q {
        let kachi = (s[r] - s[l-1]) as f32;
        let do_num = (r - (l-1)) as f32;
        if kachi > do_num / 2.0 {
            println!("win");
        } else if kachi  < do_num / 2.0 {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
