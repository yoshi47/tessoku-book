use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: [(usize, usize, usize, usize)],
    }
    let mut z = vec![vec![0_usize; w+1]; h+1];

    for i in 1..h+1 {
        for j in 1..w+1 {
            z[i][j] = z[i][j-1] + x[i-1][j-1];
        }
    }
    for j in 1..w+1 {
        for i in 1..h+1 {
            z[i][j] = z[i-1][j] + z[i][j];
        }
    }
    // println!("{:?}", z);

    for (a, b, c, d) in q {
        let ans = z[c][d] + z[a-1][b-1] - z[c][b-1] - z[a-1][d];
        println!("{ans}");
    }
}
