use proconio::input;

fn main() {
    input! {
        mut s: String
    }

    s.retain(|c| c == '1');
    println!("{:?}", s.len())
}
