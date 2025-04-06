#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(s: &String) -> char {
    s.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut s: String) {
    s = s.to_uppercase();

    println!("{s}");
}

fn main() {
    let s = "Rust is great!".to_string();

    get_char(&s);

    string_uppercase(s);
}
