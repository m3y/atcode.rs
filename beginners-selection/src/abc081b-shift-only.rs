use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    // 整数がすべて偶数であるとき、2で割ったものに置き換える
    // この処理を行える回数を求める
    //
    // これは、渡された要素の2で割れる回数の一番小さい数を答えることと一緒
    // 2で割れる回数は2進数の0が下何桁まで並んでいるかということと一緒
    // 例)
    // 8 -> 4 -> 2 -> 1 ... 3回
    // 8 -> 1000 ... 3つ並んでいる
    // 40 -> 20 -> 10 -> 5 ... 3回
    // 40 -> 101000 ... 3つ並んでいる
    //
    // rust だと以下のメソッドが利用できる
    // https://doc.rust-lang.org/std/primitive.i32.html#method.trailing_zeros
    println!("{}", a.iter().map(|e| e.trailing_zeros()).min().unwrap());
}
