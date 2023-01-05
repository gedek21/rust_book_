use std::mem::size_of_val;

pub fn _borrowing_and_refrence() {
    let s = String::from("Hello, World");
    let len = _len(&s);
    println!("{} {}", s, len);
}

fn _len(s: &String) -> usize {
    return s.len();
}

pub fn _change_refrence() {
    let mut s = String::from("Hello, ");
    _change_some_value(&mut s);
}

fn _change_some_value(s: &mut String) {
    s.push_str(" What");
}

pub fn _ownership_function() {
    let s = String::from("Heloo Rustaceans");
    _take_ownership(s);

    let i = 5;
    _make_copy(i);
}

fn _take_ownership(_some_string: String) {
    println!("{}", _some_string);
} // memory akan bebas teselah fungsi ini dijalankan, setelah melewati scope ini maka variable langsung di `drop`

fn _make_copy(_some_integer: u32) {
    println!("{_some_integer}");
} // _some_integer telah melawati scope ini dan tidak terjadi apa-apa


pub fn _calculated_string_without_refrence() {
    let s1 = String::from("Markov is return");
    let (s2, len) = _len_calculate_without_refrence(s1);

    println!("{} {}", s2, len);
}// fungsi mengkalulasi panjang string tanpa menggunakan refrence

fn _len_calculate_without_refrence(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
} // fungsi memproses panjang string dan return sebagai tuple atau list

pub fn _caluclated_string() {
    let s1 = String::from("Markov is return");
    let len = _len_calculate_with_refrence(&s1);

    println!("{s1} {len}")
}

fn _len_calculate_with_refrence(s: &String) -> usize {
    // s adalah refrence dari String
    return s.len();
}

