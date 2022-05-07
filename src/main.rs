use std::io;

fn CelciusToFahrenheit(celcius: f32) -> f32 {
    return celcius * 9.0 / 5.0 + 32.0;
}

fn CelciusMenu() {
    println!("Enter a temperature in Celcius:");
    let mut celcius = String::new();
    io::stdin()
        .read_line(&mut celcius)
        .expect("Failed to read line");
    let celcius: f32 = match celcius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };
    println!("{} Celcius is {} Fahrenheit", celcius, CelciusToFahrenheit(celcius));
}

fn Fahrenheit_to_Celsius(f: f64) -> f64 {   
    return (f - 32.0) * 5.0 / 9.0;
}


fn get_input(string: String) -> f64 {
    loop {
        println!("Please enter a number:");
        println!("{}", string);
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        let input: f64 = match input.trim().parse() {
            Ok(input) => input,
            Err(_) => continue,   
        };
        return input;
    }
}

fn Fahrenheit_Menu() {
    let f = get_input(String::from("Enter a temperature in Fahrenheit:"));
    let c = Fahrenheit_to_Celsius(f);
    println!("{}", c);  
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
                CelciusMenu();
            },
            2 => { 
                println!("===Fahrenheit===");
                Fahrenheit_Menu();

            },
            3 => { break; },
            _ => { continue; },
        }
    }
}
