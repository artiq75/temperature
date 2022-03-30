fn main() {
    println!("Temperature Converter");

    let scale = get_temperature_scale();

    if scale == 'f' {
        on_fahrenheit();
    } else if scale == 'c' {
        on_celsius();
    }

    println!("Goodbye! :)")
}

fn on_fahrenheit() {
    println!("Fahrenheit:");
    let fahrenheit = get_temperature();
    let celsius = to_celsius(fahrenheit).round();
    println!("{}°F in celsius: {}°C", fahrenheit, celsius);
}

fn on_celsius() {
    println!("Celsius:");
    let celsius = get_temperature();
    let fahrenheit = to_fahrenheit(celsius).round();
    println!("{}°C in fahrenheit: {}°F", celsius, fahrenheit);
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    input.trim().to_string()
}

fn get_temperature_scale() -> char {
    loop {
        println!("Enter 'f'(fahrenheit) or 'c'(celsius):");
        match get_input().to_lowercase().parse::<char>() {
            Ok(s) => {
                if !s.is_alphabetic() {
                    println!("Please enter a alphabetic character");
                    continue;
                }

                if s != 'f' && s != 'c' {
                    println!("Unknown value!");
                    continue;
                }

                break s;
            }
            Err(_) => {
                println!("Please enter a single character!");
                continue;
            }
        }
    }
}

fn get_temperature() -> f64 {
    loop {
        match get_input().parse() {
            Ok(t) => break t,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        }
    }
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}
