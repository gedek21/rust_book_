use std::io;

struct Termometer {
    fahrenheit: f32,
    celsius: f32,
    kelvin: f32
}

const _DEFAULT_TERMOMETER_VALUE: f32 = 273.15;

impl Termometer {
    fn _kelvin_to_celsius(&self) -> f32 {
        return self.kelvin - _DEFAULT_TERMOMETER_VALUE;
    }

    fn _kelvin_to_fahrenheit(&self) -> f32 {
        return (self.kelvin - _DEFAULT_TERMOMETER_VALUE) * 9.00/5.00 + 32.00;
    }

    fn _celsius_to_kelvin(&self) -> f32 {
        return self.celsius + _DEFAULT_TERMOMETER_VALUE;
    }

    fn _celsius_to_fahrenheit(&self) -> f32 {
        return (self.celsius * 9.00/5.00) + 32.00;
    }

    fn _fahrenheit_to_kelvin(&self) -> f32 {
        return (self.fahrenheit - 32.00) * 5.00/9.00 + _DEFAULT_TERMOMETER_VALUE
    }

    fn _fehrenheit_to_celsius(&self) -> f32 {
        return (self.fahrenheit - 32.00) * 5.00/9.00;
    }
}


// fungsi untuk menginput float mirip seperti fungsi python input(message)
// hanya terbatas untuk tipe vatiable float 32
fn _input_float(message: &str) -> f32 {
    loop {
        let mut input = String::new();
        println!("Input numbers please {message}");
        io::stdin().read_line(&mut input).expect("Cant read line");

        let _input_numbers: f32 = match input.trim().parse() {
            Ok(num_float) => { return num_float },
            Err(_) => continue
        };
    }
}

//print fungsi
pub fn _print_info() {
    let termomater_1 = Termometer {
        fahrenheit: _input_float("Fahrenheit"),
        celsius: _input_float("Celsius"),
        kelvin: _input_float("Kelvin")
    };

    println!("celsius {} to kelvin {}", termomater_1.celsius, termomater_1._celsius_to_kelvin());
    println!("fahrenheit {}", termomater_1.fahrenheit);
}