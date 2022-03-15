fn main() {
    println!("Temperature Converter");
    
    let scale = get_temperature_scale();

    if scale == 'f' {
        println!("Fahrenheit:");
        let fahrenheit = get_temperature();
        let celsius = to_celsius(fahrenheit).round();
        println!("{}° C in celsius: {}° C", fahrenheit, celsius);
    } else if scale == 'c' {
        println!("Celsius:");
        let celsius = get_temperature();
        let fahrenheit = to_fahrenheit(celsius).round();
        println!("{}° C in fahrenheit: {}° C", celsius, fahrenheit);
    }

    println!("Goodbye! :)")
}

fn get_temperature_scale() -> char {
    loop {
        println!("Enter 'f'(fahrenheit) or 'c'(celsius):");
        let mut scale = String::new();

        match std::io::stdin().read_line(&mut scale) {
            Ok(_) => {},
            Err(_) => {
                println!("Operating system error!");
                break '\0'
            },
        };

        let scale: char = match scale.trim().to_lowercase().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("Please enter a single character!");
                continue;
            }
        };

        if !scale.is_alphabetic() {
            println!("Please enter a alphabetic character");
            continue;
        }

        if scale != 'f' && scale != 'c' {
            println!("Unknown value!");
            continue;
        }

        if scale == 'f' {
            break scale
        } else {
            break scale
        }
    }
}

fn get_temperature() -> f64 {
    let mut temperature = String::new();

    loop {
        match std::io::stdin().read_line(&mut temperature) {
            Ok(_) => {},
            Err(_) => {
                println!("Operating system error!");
                break 0.0
            },
        };

        match temperature.trim().parse::<f64>() {
            Ok(t) => break t,
            Err(_) => println!("Please enter a number!")
        };
    }
}

fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}
