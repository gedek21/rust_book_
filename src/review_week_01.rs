use std::io;

struct Calculator {
    x: u32,
    y: u32,
}

impl Calculator {
    fn Add(&self) -> u32 {
        return self.x + self.y;
    }
    fn Subtraction(&self) -> u32 {
        return self.x - self.y;
    }
    fn Miltiplication(&self) -> u32 {
        return self.x * self.y;
    }
    fn Divide(&self) -> f32 {
        return (self.x / self.y) as f32
    }
}

fn input_integer(message: &str) -> u32 {
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Cant read line");

        let input_number: u32 = match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue
        };
    }
}

pub fn CalculatorApp() {
    loop {
        println!("Enter the operator");
        let mut menu = String::new();
        io::stdin().read_line(&mut menu).expect("Can read line");
        let convert_as_string: &str = menu.as_str().trim();

        let nums = Calculator {
            x: input_integer("Enter x number"),
            y: input_integer("Enter y number")
        };

        match convert_as_string {
            "add" => return println!("{} + {} = {}", nums.x, nums.y, nums.Add()),
            "sub" => return println!("{} - {} = {}", nums.x, nums.y, nums.Subtraction()),
            "multi" => return println!("{} * {} = {}", nums.x, nums.y, nums.Miltiplication()),
            "div" => return println!("{} / {} = {}", nums.x, nums.y, nums.Divide()),
            _ => continue,
        }
    }
}