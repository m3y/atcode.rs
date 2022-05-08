use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        d: [i32; n],
    }

    let set: HashSet<i32> = d.iter().cloned().collect();

    println!("{}", set.len());
}
