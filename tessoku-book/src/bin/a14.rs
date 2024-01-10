use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }
    let mut p = Vec::new();
    for x in 0..n {
        for y in 0..n {
            p.push(a[x] + b[y]);
        }
    }
    let mut q = Vec::new();
    for x in 0..n {
        for y in 0..n {
            q.push(c[x] + d[y]);
        }
    }

    q.sort();
    for i in 0..n*n {
        let pos1 = q.binary_search(&(k-p[i]));
        if pos1.is_ok() {
            println!("Yes");
            return
        }
    }
    println!("No");
}
