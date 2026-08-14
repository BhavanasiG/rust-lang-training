use std::io;

// Fibonacci sequnce: F[N+1] = F[N] + F[N-1], F[0] = 0, F[1] = 1
fn main() {
    println!("This program will generate the nth Fibonacci sequence number.");
    println!("Please enter the nth Fibonacci sequence number you will like to generate:");

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Unable to read input");

    let nth_number: u32 = nth_number
        .trim()
        .parse()
        .expect("Unable to parse input");

    const FIRST_FIB_NUMBER: u8 = 0;
    const SECOND_FIB_NUMBER: u8 = 1;
    
    if nth_number == 0 {
        return println!("The {nth_number} Fibonacci sequence number is {FIRST_FIB_NUMBER}");

    }

    if nth_number == 1 {
        return println!("The {nth_number} Fibonacci sequence number is {SECOND_FIB_NUMBER}");
    }

    let mut counter = nth_number - 2;
    
    let mut f_n_plus_1 = 0; // Represents the current sequence number generated
    let mut f_n = SECOND_FIB_NUMBER;
    let mut  f_n_minus_1= FIRST_FIB_NUMBER;

    while counter >= 0 {
        f_n_plus_1 = f_n + f_n_minus_1;
        // Set values ready to generate the next number in the sequence
        f_n_minus_1 = f_n;
        f_n = f_n_plus_1;

        counter = if counter > 0 { counter - 1} else {break};
    }
        
    println!("The {nth_number} Fibonacci sequence number is {f_n_plus_1}");
}