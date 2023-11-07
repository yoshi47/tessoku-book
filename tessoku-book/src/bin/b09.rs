use proconio::input;
fn main() {
    input! {
        z: [(usize, usize, usize, usize)],
    }
    let mut zz = vec![vec![0_isize; 1502]; 1502];
    for (a, b, c, d) in z {
        zz[a][b] += 1;
        zz[c][d] += 1;
        zz[a][d] -= 1;
        zz[c][b] -= 1;
    }

    let mut map_n = vec![vec![0_isize; 1503]; 1503];
    for i in 1..1503 {
        for j in 1..1503 {
            map_n[i][j] = map_n[i][j-1] + zz[i-1][j-1];
        }
    }
    for j in 1..1503 {
        for i in 1..1503 {
            map_n[i][j] += map_n[i-1][j];
        }
    }
    // println!("{:?}", map_n);
    // let ans: usize = map_n
    // .iter()
    // .map(|row| row.iter().filter(|&&value| value >= 1).count())
    // .sum();
    // println!("{ans}");
    let mut ans = 0;
    for i in 0..1503 {
        for j in 0..1503 {
            if map_n[i][j] >= 1 {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
