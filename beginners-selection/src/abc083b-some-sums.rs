use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    // もともと書いたコードACは取れる
    //
    //let mut ans = 0;
    //for i in 0..=n {
    //    let mut sum = i % 10;
    //    let mut tmp = i;
    //    while tmp / 10 > 0 {
    //        tmp = tmp / 10;
    //        sum += tmp % 10;
    //    }
    //    if sum >= a && sum <= b {
    //        ans += i;
    //    }
    //}

    // より rustらしい？書き方
    // https://qiita.com/tubo28/items/e6076e9040da57368845#%E7%AC%AC-5-%E5%95%8F-abc-083-b---some-sums-200-%E7%82%B9
    //
    // i32をStringにして、charのリストにした上で、
    // - char を u8 にキャスト '2' -> 50
    // - 上記を正しく計算するために、b'0'を引く
    //   - b'0' は、48
    // - b'0' を引いた値を i32にキャスト
    // - 合計することで整数の桁の合計がわかる
    let ans = (1..=n)
        .filter(|i| {
            let sum = i
                .to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as i32)
                .sum::<i32>();
            a <= sum && sum <= b
        })
        .sum::<i32>();

    println!("{}", ans);
}
