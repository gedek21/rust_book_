#[allow(dead_code)]
pub fn basic_ownership() {
    let mut a_string = String::from("Hello");
    a_string.push_str(", World");
    println!("{}", a_string);
    let s = modify_string(a_string);
    println!("{}", s);
}

fn modify_string(mut s: String) -> String {
    s.push_str(" modified");
    return s;
}

pub fn _clone_variable() {
    let s1 = String::from("Hello, World");
    let s2 = s1.clone();

    println!("s1: {}\ns2: {}", s1, s2);
}

// cloneing
fn _borrow_dog(mut dog: String) -> String {
    dog.push_str(" running");
    return dog;
}

pub fn _dog() {
    let mut _dog_name = String::from("Konco");
    _dog_name.push_str(" stacked");
    println!("My dog name is {}", _dog_name);

    let borrow_dog = _borrow_dog(_dog_name);
    println!("{}", borrow_dog)
}

fn _give_ownership() -> String {
    let s_given = String::from("Hello");
    return s_given;
}

fn _take_it_back_ownership(a_string: String) -> String {
    return a_string;
}

pub fn _basic_ownersip_2() {
    {
        let s: String = String::from("hello");
        println!("{}", s);
    }

    let x: u32 = 1;
    let y: u32 = x;

    let s1: String = _give_ownership();
    let s2: String = String::from("hello");
    let s3: String = _take_it_back_ownership(s2);

    println!("S1: {}\nS3: {}", s1, s3);
}


// refrenceses

// borrowed with out a refrenceses
fn _calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    return (s, length);
}

pub fn _basic_references() {
    let s1 = String::from("hello"); // 5kb of memory
    let (s2, len) = _calculate_length(s1); // not taking memory because borrowing a value from s1
    println!("{s2} {len}"); // when its done it will automatically clean it self
}

// refactor a code
fn _calculate_length_refactor(s: &String) -> usize {
    return s.len();
}

pub fn _refrenceses_refactor() {
    let s1 = String::from("hello");
    let len = _calculate_length_refactor(&s1);

    println!("{} {}", s1, len);
}

pub fn _owner_and_refrence() {
    let x = 5;
    let y = &x;

    println!("x(original owner value) : {}\ny(reference of value x and derefence of this value): {} ", x, *y)
}

pub fn _mutable_refrences() {
    let mut a_string = String::from("Hello, ");
    _change(&mut a_string);

    println!("{}", a_string);
}

fn _change(some_string: &mut String) {
    some_string.push_str("World!!");
}

