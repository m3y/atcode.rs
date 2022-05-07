use proconio::input;

fn main() {
    input! {
        ab: [i32; 2],
    }

    println!(
        "{}",
        if (ab[0] * ab[1]) % 2 == 0 {
            "Even"
        } else {
            "Odd"
        }
    );
}
