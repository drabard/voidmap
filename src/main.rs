use std::io;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    
    io::stdin().read_line(&mut first_number).
      expect("Failed to read line.");
    let first_number: i32 = first_number.trim().parse().expect("Failed to parse number.");

    io::stdin().read_line(&mut second_number).
      expect("Failed to read line.");
    let second_number: i32 = second_number.trim().parse().expect("Failed to parse number.");

    println!("First number: {}, second number: {}", first_number, second_number)
}