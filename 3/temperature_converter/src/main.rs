use std::io;
use std::io::Write; //so I can use flush()
use std::str::FromStr;

enum TemperatureUnits {
    Celsius(f32),
    Fahrenheit(f32),
    Kelvin(f32)
}

//Could I implement some kind of FromChar?
//I saw someone implementing Mask, which seems similar, but I'm not sure
impl FromStr for TemperatureUnits {
    type Err = ();

    fn from_str(input: &str) -> Result<TemperatureUnits, Self::Err> {
        match input {
            "C" => Ok(TemperatureUnits::Celsius(0.0)),
            "F" => Ok(TemperatureUnits::Fahrenheit(0.0)),
            "K" => Ok(TemperatureUnits::Kelvin(0.0)),
            _   => Err(()),
        }
    }
}

fn main() {
    
    println!("Welcome to temperature converter 5000!");

    let source = get_unit_from_user("From".to_string());
    let target = get_unit_from_user("To".to_string());
}

fn get_unit_from_user(which_unit: String) -> TemperatureUnits {
    let source_unit: TemperatureUnits;

    let mut user_input = String::new();
    loop {
        user_input.clear();

        println!("{which_unit} which unit would you like to convert? C, F or K");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        source_unit = match TemperatureUnits::from_str(&user_input.to_uppercase()) {
            Ok(unit) => unit,
            Err(_) => {
                println!("No temperature unit related to {user_input}. Options: C, F or K");
                continue;
            },
        };

        break;
    }
    return source_unit;
}

fn fahrenheit_to_celsius(temperature: TemperatureUnits) -> TemperatureUnits {
    let mut value: f32 = 0.0;
    if let TemperatureUnits::Fahrenheit(mut value) = temperature {
        //((F-32)*5)/9
        value = ((value - 32.0) * 5.0) / 9.0;
    } else {
        panic!("Incorrect Temperature unit type");
    }
    return TemperatureUnits::Celsius(value);
}

fn celsius_to_kelvin(temperature: TemperatureUnits) -> TemperatureUnits {
    let mut value: f32 = 0.0;
    if let TemperatureUnits::Celsius(mut value) = temperature {
        //k = c + 273.15
        value = value + 273.15;
    } else {
        panic!("Incorrect Temperature unit type");
    }
    return TemperatureUnits::Kelvin(value);
}

fn kelvin_to_fahrenheit(temperature: TemperatureUnits) -> TemperatureUnits {
    let mut value: f32 = 0.0;
    if let TemperatureUnits::Kelvin(mut value) = temperature {
        //f = ((k-273.15)*9)/5
        value = ((value - 273.15) * 9.0) / 5.0;
    } else {
        panic!("Incorrect Temperature unit type");
    }
    return TemperatureUnits::Fahrenheit(value);
}
