use std::io;

fn main() {

    let mut temperature_value_string = String::new();
    let mut temperature_unit_string = String::new();
    let temperature_value :i32;
    let mut temperature_unit :char = ' ';
    let converted_value :i32;

    
    loop {
        println!("Please enter temperature value:");
        
        temperature_value_string.clear();
        io::stdin().read_line(&mut temperature_value_string)
            .expect("Failed to read line");
        
        match temperature_value_string.trim().parse::<i32>() {
            Ok(val) => {
                temperature_value = val;
                break;
            },
            Err(_) => {
                println!("Please enter a valid integer value.");
            },
        }
    }

    while temperature_unit.to_ascii_uppercase() != 'C' && temperature_unit.to_ascii_uppercase() != 'F'  {
        println!("Please enter temperature unit as C or F:");

        temperature_unit_string.clear();
        io::stdin().read_line(&mut temperature_unit_string)
            .expect("Failed to read line");

        match temperature_unit_string.trim().to_ascii_uppercase().chars().next() {
            Some('C') => temperature_unit = 'C',
            Some('F') => temperature_unit = 'F',
            _ => println!("Please enter unit as C or F."),
        }
    }

    if(temperature_unit.to_ascii_uppercase() == 'C') {
        converted_value = CelsiustoFahrenheit(temperature_value);
    }
    else{converted_value = FahrenheittoCelsius(temperature_value);} 

    println!("Converted value is: {}", converted_value);

}

fn CelsiustoFahrenheit(x : i32) -> i32{
    let y :i32;
    y = x * 9 / 5 +32;
    return y;
}

fn FahrenheittoCelsius(x : i32) -> i32{
    let y :i32;
    y= (x-32) *5 / 9;
    return y;
}