use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    while !s.is_empty() {
        if s.ends_with("eraser") {
            s = s.trim_end_matches("eraser").to_string();
        } else if s.ends_with("erase") {
            s = s.trim_end_matches("erase").to_string();
        } else if s.ends_with("dreamer") {
            s = s.trim_end_matches("dreamer").to_string();
        } else if s.ends_with("dream") {
            s = s.trim_end_matches("dream").to_string();
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
