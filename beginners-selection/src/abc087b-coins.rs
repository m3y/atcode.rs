use proconio::input;

fn main() {
    input! {
        a: i32,  // 500円玉の枚数
        b: i32,  // 100円玉の枚数
        c: i32,  // 50円玉の枚数
        x: i32,  // 求める合計金額
    }

    let mut ans = 0;

    for an in 0..=a {
        for bn in 0..=b {
            for cn in 0..=c {
                if an * 500 + bn * 100 + cn * 50 == x {
                    ans += 1
                }
            }
        }
    }

    println!("{}", ans);
}
