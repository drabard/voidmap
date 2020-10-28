use std::io;
use std::cmp::Ordering;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    
    io::stdin().read_line(&mut first_number).
      expect("Failed to read line.");
    let first_number: i32 = first_number.trim().parse().expect("Failed to parse number.");

    io::stdin().read_line(&mut second_number).
      expect("Failed to read line.");
    let second_number: i32 = second_number.trim().parse().expect("Failed to parse number.");

    match first_number.cmp(&second_number) {
      Ordering::Less => println!("First number is less than second number."),
      Ordering::Greater => println!("First number is greater than second number."),
      Ordering::Equal => println!("The numbers are equal.")
    }
}