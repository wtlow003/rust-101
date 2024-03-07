use std::io;

#[allow(dead_code)]
fn temperature_converter(f: Option<f64>, c: Option<f64>) {
    match (f, c) {
        (Some(f), None) => {
            let c = (f - 32.0) * 5.0 / 9.0;
            println!("{}°F is {}°C", f, c);
        }
        (None, Some(c)) => {
            let f = c * 9.0 / 5.0 + 32.0;
            println!("{}°C is {}°F", c, f);
        }
        _ => println!("You must provide exactly one argument"),
    }
}

fn main() {
    println!("Please enter a temperature in Fahrenheit or Celsius with the unit (e.g. 32F or 0C):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // remove whitespace characters
    input = input.trim().to_string();
    println!("You entered: {}", input.trim());

    if input.trim().len() < 2 {
        println!("You must provide a temperature and a unit");
        return;
    }
    let unit = input.chars().last().unwrap().to_uppercase().to_string();
    println!("Unit: {}", unit);

    if unit == "F" {
        let f: f64 = input[..input.len() - 1].parse().unwrap();
        temperature_converter(Some(f), None);
    } else if unit == "C" {
        let c: f64 = input[..input.len() - 1].parse().unwrap();
        temperature_converter(None, Some(c));
    } else {
        println!("Invalid unit");
    }
}
