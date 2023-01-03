use std::io;
use std::str::FromStr;
use std::env;

#[allow(dead_code)]
pub fn magic_numbers() {
    const GRAVITY: f32 = 9.81;
    const NOT_CHANGE_NUMBER: u32 = 0;

    let some_number = 10; // variable tidak dapat bermutasi
    println!("number is : {some_number}");

    let mut change_number = 0; // variable dapat bermutasi
    println!("current number is {change_number}");
    change_number = 100;
    println!("now number is change {change_number}");

    // shadowing number in the inner scope
    {
        let some_number: u32 = change_number * 10;
        println!("shadow number in the inner scope {some_number}")
    }

    let guess: u32 = "42".parse().expect("not number");
    print!("{guess}");

    let _pi = 3.14; // default float64
    let _money: f32 = 1000.00;

    let _sum_number = 10 + 3;
    let _sub_number = 210.00 - 100.21;
    let _mul_number = 10 * 10;
    let _div_number = 10 / 5;
    let _remainder = 10 % 2 == 0;

    print!("{_sum_number}\n{_sub_number}\n{_mul_number}\n{_div_number}\n{_remainder}\n");

    let _is_true: bool = false;
    let _is_fizz: bool = 2 % 3 == 0;
    let _is_fizz_buzz: bool = 15 % 5 == 0 && 15 % 3 == 0;

    println!("is fizz buzz = {_is_fizz_buzz}");

    let _z: char = 'Z';
    let _c: char = 'H';
    let _emote: char = '😻';

    let _tups: (u32, bool, &str) = (12, false, "Yeah");
    let (_tup_number, _tup_bool, _tup_str) = _tups;

    println!("list tups : {_tup_number}, {_tup_bool}, {_tup_str}");

    let _list_todo: (&str, &str, &str) = ("Task 1", "Task 2", "Task 3");
    let _todo_1 = _list_todo.0;
    println!("tuple Todo list : {_todo_1}");

    let _list_programming: [&str; 6] = ["Javasciprt/Typescript", "C/C++", "Go", "Rust", "Python", "Java"];
    for _programming in _list_programming  {
        println!("{_programming}");
    }

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index range is not a number");

    let element = a[index];
    println!("The value of the element at index {index} is {element}");
}


// Excercises
#[allow(dead_code)]
pub fn excercises_basic_variable() {
    let _x: u32 = 100;
    let _f: f32 = 100.90;
    let _s: &str = "Rust Programming";
    let _c: char = 'C';
    let _tup: (i32, u32, f32) = (2, 3, 3.14);
    let _array = ["Javascript", "Rust", "Go"];

    let _tup_one = _tup.0;
    let _tup_two = _tup.1;
    let _tup_three = _tup.2;

    print!("Integer: {_x}\nFloat: {_f}\nString: {_s}\nChar: {_c}\nTuple: {_tup_one}, {_tup_two}, {_tup_three}\nArray: {:?}", _array);
}

// Excercises basic input output
#[allow(dead_code)]
pub fn info_users() {

    println!("Enter your name : ");
    let mut _name = String::new();
    io::stdin().read_line(&mut _name).expect("Input name please?");


    println!("Enter your age : ");
    let mut input_line_age = String::new();
    io::stdin().read_line(&mut input_line_age).expect("Failed to read line");
    let mut _age: u32 = input_line_age.trim().parse().expect("Input not an integer");

    println!("Enter your height : ");
    let mut input_line_height = String::new();
    io::stdin().read_line(&mut input_line_height).expect("Failed to real line");
    let mut _height: f32 = input_line_height.trim().parse().expect("Input not an float");

    let mut _email = String::new();
    println!("Enter your email");
    io::stdin().read_line(&mut _email).expect("Input is not an String");

    print!("Name is {_name}\nAge is {_age}\nHeight is {_height}\nEmail is {_email}");
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return  n;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_ne!(gcd(2 * 3 * 5 * 11 * 17,
                    3 * 7 * 11 * 13 * 19), 3 * 11);
}


pub fn _advance_basic_variable() {
    let mut _numbers = Vec::new();

    for arg in env::args().skip(1) {
        _numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if _numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBERS....");
        std::process::exit(1);
    }

    let mut d = _numbers[0];
    for m in &_numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", _numbers, d);

}