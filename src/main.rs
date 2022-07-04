use std::io;
fn main() {
    
    let mut input;
    let mut initial_measuring_unit: char;
    let mut final_measuring_unit: char;
    
    let value: f64;

    println!("Enter the temperature you want to convert (format [value-measuring unit])");


    value = loop {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        input = input.trim().to_string();
        input = input.to_lowercase().to_string();

        initial_measuring_unit = input.chars().last().unwrap();
        input.pop();

        if !"fck".contains(initial_measuring_unit) {
            println!("Input has to be in correct format [value-measureing unit] (ex: 100C, 55F, 300K). Enter a valid format: ");
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(t) => {
                break t;
            }
            Err(_) => {
                println!("Input has to be in correct format [value-measureing unit] (ex: 100C, 55F, 300K). Enter a valid format: ");
            }
        };
    };

    println!("Which unit do you want to convert into? (F, C, K)");

    loop {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        final_measuring_unit = input.chars().nth(0).unwrap().to_ascii_lowercase();

        if ("fck".contains(final_measuring_unit)) && input.chars().count() < 4{
            break;
        }
    }

    let converted_value: f64;

    match (initial_measuring_unit, final_measuring_unit) {
        ('c', 'f') => converted_value = (value - 32.0) / 5.0 / 9.0,
        ('c', 'k') => converted_value = value + 273.15,
        ('f', 'c') => converted_value = value * 5.0 / 9.0 + 32.0,
        ('f', 'k') => converted_value = (value - 32.0) * 5.0 / 9.0 + 273.15,
        ('k', 'c') => converted_value = value - 273.15,
        ('k', 'f') => converted_value = (value - 273.15) * 9.0 / 5.0 + 32.0,
        _ => converted_value = value
    };

    println!("{}{} equals {:.1}{}", value, initial_measuring_unit.to_uppercase(), converted_value, final_measuring_unit.to_uppercase());

}
