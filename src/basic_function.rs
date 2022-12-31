use std::io;

fn add(x: u32, y: u32) -> u32 {
    return x + y;
}

fn print_function() {
    println!("Enter first number : ");
    let mut input_lane_first_number = String::new();
    io::stdin().read_line(&mut input_lane_first_number).expect("Failed to read first number line");
    let _num_1: u32 = input_lane_first_number.trim().parse().expect("Error the input is not number");

    println!("Enter second number : ");
    let mut input_lane_second_number = String::new();
    io::stdin().read_line(&mut input_lane_second_number).expect("Failed to read second number line");
    let _num_2: u32 = input_lane_second_number.trim().parse().expect("Error the input is not number");

    let result = add(_num_1, _num_2);

    println!("Result : {result}");
}

#[allow(dead_code)]
pub fn basic_function_print_info() {
    print_function();
}