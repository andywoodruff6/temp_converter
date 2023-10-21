// By Woodruff 07-09-2023

use std::io;

fn main() {
    
    // get scale
    let mut scale: String = String::new();
    println!("To convert a temp, type the letter of the temp you have. F for farenheight or C for celcius:");
    io::stdin()
        .read_line(&mut scale).expect("Error");

    let clean_scale = scale.trim();
    println!("You have chosen {}",clean_scale);

    //get number
    let mut input: String = String::new();
    println!("add input as number");
    io::stdin()
        .read_line(&mut input).expect("Failed to read line");

    println!("input was: {input}");
    
    let input_as_number: i32;

    //input_as_number = input.trim().parse::<i32>().unwrap();
    match input.trim().parse::<i32>() {
        Ok(n) => input_as_number = n,
        Err(e) => panic!("input is not an i32: {}", e)
      }


    match &clean_scale[..] {
        "F" => {
            let output: i32 = (input_as_number - 32) * 5 / 9;
            println!("Temp in Celcius is {output}.")
        },
        "C" => {
            let output: i32 = (input_as_number + 32) * 9 / 5;
            println!("Temp in Farenheight is {output}.")
        },
        _ => println!("Temperature Scale issue.")
    };

}