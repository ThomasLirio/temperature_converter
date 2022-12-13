
use std::io;
const F: char = 'F';
const C: char = 'C';

fn main() {

    println!("Conversion between Fahrenheit and Celsius");

    println!("Please input the temperature to convert and its original scale: ");

    println!("({F} for Fahrenheit, {C} for Celsius)");

    println!("Examples: 45.32{F} or 12{C} or {C}32.8 or {F}30");

    let mut temp2convert: String = String::new();
    io::stdin()
        .read_line(&mut temp2convert)
        .expect("failed to read line");
     
    match convert(&temp2convert) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("we don´t know how to convert {temp2convert}: {e}"),
    }
}

fn convert(temp2convert: &str) -> Result<String, &'static str> { 
    let temp = temp2convert.trim().to_uppercase();
    if temp.len()<=1 {
        return Err("we don´t know how to convert {temp}, please input something like 45.32F or 12C or C32.8 or F30");
    }
    
    
    let tmp: &str;
    if temp.ends_with(F) || temp.ends_with(C) {
        tmp = temp.get(0..temp.len()-1).unwrap().trim();
    } else if temp.starts_with(F) || temp.starts_with(C) {
        tmp = temp.get(1..temp.len()).unwrap().trim();
    } else {
        return Err("we don´t know how to convert {temp}");
    }

    let degree: f32;
    match convert2degrees(tmp){
        Ok(x) => degree = x,
        Err(e) =>  return Err(e),
    };

    if temp.ends_with(F) || temp.starts_with(F){
        convert2celsius(degree)
    } else if temp.ends_with(C) || temp.starts_with(C) {
        convert2fahrenheit(degree)
    }  
    else {
        return Err("we don´t know how to convert {temp}");
    }
}


fn convert2degrees(temp: &str) -> Result<f32, &'static str> {
    match temp.parse::<f32>(){
        Ok(x) => Ok(x),
        Err(_) => Err("besides the scale, please input a number"),
    }   
}

fn convert2celsius(degree: f32) -> Result<String, &'static str> {
    
    const FAHRENHEIT_MIN: f32 = -459.67;
    const FAHRENHEIT_MAX: f32 = 212.0;
    match degree {
        x if x < FAHRENHEIT_MIN => Err("it is freeeeeeeezing! below absolute zero!"),
        x if x <= FAHRENHEIT_MAX => {
            let celsius = round((degree-32.0)*5.0/9.0,2);

            let output = format!("{degree} Fahrenheit is {celsius} Celsius.");
 
            Ok(output)
        },
        x if x > FAHRENHEIT_MAX => Err("Too hot to handle the convertion!"),
        _ => Err("we don´t know how to convert {degree}")
    }
}

fn convert2fahrenheit(degree: f32) -> Result<String, &'static str> {
    
    const CELSIUS_MIN: f32 = -273.15;
    const CELSIUS_MAX: f32 = 100.0;
    match degree {
        x if x < CELSIUS_MIN => Err("it is freeeeeeeezing! below absolute zero!"),
        x if x <= CELSIUS_MAX => {
            let fahrenheit = round((degree*9.0/5.0)+32.0,2);

            let output = format!("{degree} Celsius is {fahrenheit} Fahrenheit.");
 
            Ok(output)
        },
        x if x > CELSIUS_MAX => Err("Too hot to handle the convertion!"),
        _ => Err("we don´t know how to convert {degree}")
    }
}

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}