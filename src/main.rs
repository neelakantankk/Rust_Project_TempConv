use std::io;

fn main() {
    println!("Temperature Conversion!\n");
    loop {

        let mut temperature = String::new();
        let mut choice = String::new();

        println!("What do you want to do?");
        println!("1. Convert from Fahrenheit to Celsius");
        println!("2. Convert from Celsius to Fahrenheit");
        println!("Enter your choice (1/2/Q to quit)");
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line!");

        let choice : u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if choice != 1 && choice != 2 {
            break;
        }
        println!("Enter the temperature: ");
        
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line!");

        let temperature : f32 = temperature.trim().parse()
            .expect("You need to enter a number.");

        if choice == 1 {
            println!("\nThe temperature in Celsius is {}\n", fahrenheit_to_celsius(temperature));
        } else if choice == 2 {
            println!("The temperature in Celsius is {}\n", celsius_to_fahrenheit(temperature));
        }
    }
    println!("Bye!");
}

fn fahrenheit_to_celsius(temperature : f32) -> f32 {
    (temperature - 32.0)/1.8
}

fn celsius_to_fahrenheit(temperature : f32) -> f32{
    (temperature * 1.8) + 32.0
}
