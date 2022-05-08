use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut s: String = s.chars().rev().collect();

    while !s.is_empty() {
        if s.starts_with("resare") {
            s = s.trim_start_matches("resare").to_string();
        } else if s.starts_with("esare") {
            s = s.trim_start_matches("esare").to_string();
        } else if s.starts_with("remaerd") {
            s = s.trim_start_matches("remaerd").to_string();
        } else if s.starts_with("maerd") {
            s = s.trim_start_matches("maerd").to_string();
        } else {
            break;
        }
    }

    if s.len() == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
