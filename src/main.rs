use std::io;

fn CelciusToFahrenheit(celcius: f32) -> f32 {
    return celcius * 9.0 / 5.0 + 32.0;
}


fn main() {
    println!("Welcome to the calculator!");

    loop {
        println!("[1] Celcius to Fahrenheit");
        println!("[2] Fahrenheit to Celcius");
        println!("[3] Quit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => { 
                println!("===Celcius===");
            },
            2 => { 
                println!("===Fahrenheit===");
            },
            3 => { break; },
            _ => { continue; },
        }
    }
}
