use proconio::input;
fn main() {
    input! {
        xy: [(usize, usize)],
        q : [(usize, usize, usize, usize)],
    }
    let mut z = vec![vec![0_usize; 1500]; 1500];
    let mut k = vec![vec![0_usize; 1501]; 1501];

    for (x, y) in xy {
        z[x-1][y-1] += 1;
    }

    for i in 1..1501 {
        for j in 1..1501 {
            k[i][j] = k[i][j-1] + z[i-1][j-1];
        }
    }
    for j in 1..1501 {
        for i in 1..1501 {
            k[i][j] += k[i-1][j];
        }
    }

    for (a, b, c, d) in q {
        let ans:usize = k[c][d] + k[a-1][b-1] - k[c][b-1] - k[a-1][d];
        println!("{ans}");
    }
}
