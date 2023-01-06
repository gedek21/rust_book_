mod basic_variable;
mod basic_ownership;
pub mod basic_function;
pub mod basic_contorl_flow;
mod borrow_ownership_refrenceses;
pub mod basic_struct;
pub mod exam_part_01;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[allow(dead_code)]
fn guessing_number() {
    println!("|======= Enter number =======|");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number}");

    loop {
        println!("Enter number please");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}

#[allow(dead_code)]
fn bridge_other_module() {
    let string_input = basic_contorl_flow::_read_input_string("Enter");

    match string_input {
        Ok(input_string) => println!("{input_string}"),
        Err(e) => println!("An error occurred: {e}")
    }
}

#[allow(dead_code)]
fn _loop_fobinacci() {
    let mut n: u32 = basic_contorl_flow::_read_input_integer("Number");
    let counter: u32 = basic_contorl_flow::_read_input_integer("Counter");
    while n < counter {
        let fib: u32 = basic_contorl_flow::_fibonacci(n);
        println!("{}", fib);
        n += 1;
        if n > counter {
            break;
        }
    }
}

fn main() {
    exam_part_01::_print_info();
}

