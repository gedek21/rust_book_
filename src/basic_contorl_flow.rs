use std::io;

pub fn _checking_login() {
    loop {
        println!("Enter user name");
        let mut _username = String::new();
        io::stdin()
            .read_line(&mut _username)
            .expect("Cannot read line or is not a String");

        println!("Enter password");
        let mut _password = String::new();
        io::stdin()
            .read_line(&mut _password)
            .expect("Cannot real line or is not a String");

        if _username.trim().is_empty() && _password.trim().is_empty() {
            println!("The user or password is empty\nReenter user and password again!!!")
        } else {
            println!("Username {_username}\nPassword {_password}");
            break;
        }
    }
}
#[allow(dead_code)]
struct KnightStats {
    strength: u32,
    intellegent: u32,
    vitality: u32,
    endurance: u32,
    wisdom: u32,
    agility: u32,
    luck: u32,
}
#[allow(dead_code)]
struct MonsterStats {
    strength: u32,
    intellegent: u32,
    vitality: u32,
    endurance: u32,
    wisdom: u32,
    agility: u32,
    luck: u32,
}

pub fn _knight_and_sword() {
    println!("Enter knight name : ");

    let mut _input_line = String::new();
    io::stdin()
        .read_line(&mut _input_line)
        .expect("Failed to read line");

    let _knight_name: &str = _input_line.trim();

    let _knight_stats: KnightStats = KnightStats {
        strength: 10,
        intellegent: 4,
        vitality: 8,
        endurance: 9,
        wisdom: 7,
        agility: 4,
        luck: 5,
    };

    let _goblin_stats:MonsterStats = MonsterStats {
        strength: 4,
        intellegent: 1,
        vitality: 2,
        endurance: 1,
        wisdom: 1,
        agility: 5,
        luck: 2,
    };

    let _can_beat_minotaur: bool = if _knight_stats.strength >= 8 && _knight_stats.vitality >= 5 {
        true
    } else {
        false
    };

    if _knight_stats.strength >= _goblin_stats.strength {
        println!("{_knight_name} can beat this goblin");
    }

    if _can_beat_minotaur {
        println!("{_knight_name} can beat minotaur");
    }
}

pub fn _looping_basic() {
    let mut _counter = 0;
    let result = loop {
        _counter += 1;
        if _counter == 5 {
            break _counter * 2;
        }
    };
    println!("{result}");
}

pub fn _lifttoff() {
//    let mut _start = 5;
//    while _start != 0 {
//        println!("{_start}");
//        _start -= 1;
//    }
    let _numbers: i32 = 10;
    for _number in (1.._numbers).rev() {
        println!("{_number}!");
    }
    println!("LIFTOFF!!!");
}

pub fn _looping_an_array() {
    let _languages = ["Typescript/Javascript", "Rust", "Python", "C/C++", "Go"];
    for _language in _languages {
        println!("{_language}");
    }
}

pub fn _input_names() {
    loop {
        println!("Enter name please");
        let mut _read_line = String::new();
        io::stdin()
            .read_line(&mut _read_line)
            .expect("Cannot read line");

        let _name: &str = _read_line.trim();

        if _name != "" {
            println!("Your name is {_name}");
            break;
        }
    }
}

fn _celsius_to_fahrenheit(cels: f32) -> f32 {
    return(cels * 9 as f32 / 5 as f32) + 32 as f32;
}

fn _fahrenheit_to_celsius(fahr: f32) -> f32 {
    return(fahr - 32 as f32) * 5 as f32 / 9 as f32;
}

#[allow(unused_variables)]
fn _read_input_float(message: &str) -> f32 {
    loop {
        let mut input_string = String::new();
        println!("Input Number {message} : ");
        io::stdin().read_line(&mut input_string).expect("Failed to read line");

        let input_number: f32 = match input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue
        };
    }
}

pub fn _read_input_integer(message: &str) -> u32 {
    loop {
        let mut _input_string = String::new();
        println!("Input Number {message} : ");
        io::stdin().read_line(&mut _input_string).expect("Cant read line propperly");

        let _input_integer: u32 = match _input_string.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue
        };
    }
}

pub fn _read_input_string(message: &str) -> io::Result<String> {
    let mut input_string = String::new();
    loop {
        println!("Input the {message}");
        io::stdin().read_line(&mut input_string).expect("Cannot read line string");
        input_string = input_string.trim().to_string();
        if !input_string.is_empty() {
            break;
        }
        println!("Enter an string again!");
    }
    Ok(input_string)
}

pub fn _fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return _fibonacci(n - 1) + _fibonacci(n - 2);
    }
}


pub fn _info_celsius_fahrenheit() {
    let mut _number: f32 = _read_input_float("Start");
    let _end: f32 = _read_input_float("End");
    let mut _step: f32 = _read_input_float("Step");

    while _number <= _end {
        let mut _celsius_convert: f32 = _fahrenheit_to_celsius(_number);
        println!("{_number}\t{_celsius_convert}");
        _number = _number + _step;

        if _number >= _end {
            break;
        }
    }
}

pub fn _the_twelve_days_of_christmas() -> String {
    let days = Vec::from([
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
        ]);
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
        ];

    let mut lyrics = String::new();
    for i in 0..days.len() {
        lyrics.push_str(&format!("On the {} day of Christmas, my true love gave to me\n", days[i]));
        for j in (0..=i).rev() {
            if j == 0 && i > 0 {
                lyrics.push_str("and ");
            }
            lyrics.push_str(&format!("{}\n", gifts[j]));
        }
        lyrics.push_str("\n");
    }
    return lyrics;
}