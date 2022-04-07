use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess your number.");
    println!("Please input your guess.");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    let mut guess: String = String::new(); // mutable
    let apples: i32 = 5; // immutable

    println!("Your secret_number is: {}", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the line");

    println!("You guessed the following thing: {}", guess);

    let x = 5;
    let y = 3;
    println!("x = {}, y = {}", x, y);

    println!("Let us do loop guessing.");

    loop {
        println!("please put your another guess.");

        println!("You guessed the following thing: {}", guess);

        let mut guess: String = String::new(); 

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
