use std::io; // To obtain user input and then print the result as output - use io input/output library. io library comes from the standard library, known as std.

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

  println!("You guessed: {guess}")
}