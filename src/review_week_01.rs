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

fn input_string(message: &str) -> io::Result<String> {
    let mut input = String::new();
    loop {
        println!("{}", message);
        io::stdin().read_line(&mut input).expect("Cant read line");
        input = input.trim().to_string();
        if !input.is_empty() {
            break;
        }
        println!("Enter an string again!");
    }
    Ok(input)
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
        let menu = input_integer("Enter menu");

        let nums = Calculator {
            x: input_integer("Enter x number"),
            y: input_integer("Enter y number")
        };

        match menu {
            1 => return println!("{} + {} = {}", nums.x, nums.y, nums.Add()),
            2 => return println!("{} - {} = {}", nums.x, nums.y, nums.Subtraction()),
            3 => return println!("{} * {} = {}", nums.x, nums.y, nums.Miltiplication()),
            4 => return println!("{} / {} = {}", nums.x, nums.y, nums.Divide()),
            _ => continue,
        }
    }
}