use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    let mut f = false;

    'outer: for a in 0..=n {
        for b in 0..=(n - a) {
            let c = n - (a + b);
            if a + b + c <= n && a * 10000 + b * 5000 + c * 1000 == y {
                f = true;
                println!("{} {} {}", a, b, c);
                break 'outer;
            }
        }
    }

    if !f {
        println!("-1 -1 -1");
    }
}
