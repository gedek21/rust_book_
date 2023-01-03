pub fn _borrowing_and_refrence() {
    let s = String::from("Hello, World");
    let len = _len(&s);
    println!("{} {}", s, len);
}

fn _len(s: &String) -> usize {
    return s.len();
}

fn _change_some_value(s: &mut String) {
    s.push_str(" What");
}

pub fn _change_refrence() {
    let mut s = String::from("Hello, ");
    _change_some_value(&mut s);
}

