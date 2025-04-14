use std::io;

fn main() {
    println!("Temp converter");
    loop {
        println!("Choose an option:");
        println!("1 - Celsius to Fahrenheit");
        println!("2 - Fahrenheit to Celsius");
        println!("0 - Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please choose a number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter Temperature in Celsius");
                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius).expect("Error");

                let celsius: f64 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };
                let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
                println!("{:.1}°C = {:.1}°F", celsius, fahrenheit);
            }
            2 => {
                println!("Enter Temperature in Fahrenheit");
                let mut fahrenheit = String::new();
                io::stdin().read_line(&mut fahrenheit).expect("Error");

                let fahrenheit: f64 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number");
                        continue;
                    }
                };
                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("{:.1}°F = {:.1}°C", fahrenheit, celsius);
            }
            0 => {
                println!("Bye bye");
                break;
            }
            _ => println!("Invalid operation"),
        }
    }
}
