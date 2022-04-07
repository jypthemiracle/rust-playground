use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn another_function(x: i32, unit_label: char) {
    println!("another function.");
    println!("the value of this, {}", x);
    println!("the value of ..., {}", unit_label);
}

fn five() -> i32 {
    // most functions return the last expression implicitly.
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn counting() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn main() {
    let x = five();
    println!("five {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    another_function(32, 'a');
    println!("Guess your number.");
    println!("Please input your guess.");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    let mut guess: String = String::new(); // mutable
    let apples: i32 = 5; // immutable

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);


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

    let guess: u32 = "42".parse().expect("Not a number!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("hey, {}", x);

    let crator = [1, 2, 3, 4, 5];

    let month:[&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let march: &str = month[2];

    let number = 3;

    if number < 5 {
        println!("condition is true")
    } else {
        println!("condition is false")
    }
}
