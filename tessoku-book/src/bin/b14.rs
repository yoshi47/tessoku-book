use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let enumerate = |x: &[usize]| {
        let mut sum_list = Vec::new();
        for i in 0..1 << x.len() {
            let mut sum = 0;
            for j in 0..x.len() {
                if i & (1 << j) != 0 {
                    sum += x[j];
                }
            }
            sum_list.push(sum);
        }
        return sum_list
    };

    let (l1, l2) = a.split_at(n / 2);
    let mut sum1 = enumerate(l1);
    let mut sum2 = enumerate(l2);
    sum1.sort();
    sum2.sort();

    for s1 in sum1 {
        match sum2.binary_search(&(k - s1)) {
            Ok(_) => {
                println!("Yes");
                return
            },
            Err(_) => continue,
        }
    }
    println!("No");
}
