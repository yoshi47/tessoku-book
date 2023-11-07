use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        z: [(usize, usize, usize, usize); n],
    }
    let mut zz = vec![vec![0_i32; w+1]; h+1];
    for (a, b, c, d) in z {
        zz[a-1][b-1] += 1;
        zz[c][d] += 1;
        zz[a-1][d] -= 1;
        zz[c][b-1] -= 1;
    }
    // 累積和の計算
    let mut zzz = vec![vec![0_i32; w+2]; h+2];
    for i in 1..h+1 {
        for j in 1..w+1 {
            zzz[i][j] = zzz[i][j-1] + zz[i-1][j-1];
        }
    }
    for j in 1..w+1 {
        for i in 1..h+1 {
            zzz[i][j] = zzz[i-1][j] + zzz[i][j];
        }
    }

    for i in 1..h+1 {
        for j in 1..w+1 {
            print!("{} ", zzz[i][j]);
        }
        println!();
    }
}
