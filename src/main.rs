use std::io;

fn main() {

    println!("This program allows you to convert from Farenheight to Celsius");
    println!("Please enter your temperature in Farenheight:");

    let mut user_farenheight = String::new();

    io::stdin()
        .read_line(&mut user_farenheight)
        .expect("Failed to read input");
    
    let user_farenheight: f32 = user_farenheight
    .trim()
    .parse()
    .expect("Input entered was not a number");

    let celcius = (user_farenheight - 32.0) * 5.0/9.0;

    println!("In Celsius that is {celcius}");
}
