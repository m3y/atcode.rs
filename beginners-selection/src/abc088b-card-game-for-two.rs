use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }

    a.sort();
    let mut alice = 0;
    let mut bob = 0;

    while !a.is_empty() {
        if let Some(e) = a.pop() {
            alice += e;
        }

        if let Some(e) = a.pop() {
            bob += e;
        }
    }

    println!("{}", alice - bob);
}
