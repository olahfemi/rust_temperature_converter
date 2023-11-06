use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter 'c' to convert Celsiun to Fahrenheit or 'f' for Fahrenhiet to Celsius");

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let final_input: &str = input.trim();
    if final_input == "c" {
        println!("Converting Celsius to Fahrenheit");
        let mut celsius = String::new();
        io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");
        let degree_celsius: f32 = celsius.trim().parse().expect("Input not an integer");
        let result = celsius_to_fahrenheit(degree_celsius);
        println!("{result}")
    } 
    else if final_input == "f" {
        println!("Converting Fahrenhiet to Celsius");
        let mut fahrenheit = String::new();
        io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
        let degree_fahrenheit: f32 = fahrenheit.trim().parse().expect("Input not an integer");
        let result = fahrenheit_to_celsius(degree_fahrenheit);
        println!("{result}")
    }
    else {
        println!("You entered a wrong letter")
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.) * 0.56
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}
