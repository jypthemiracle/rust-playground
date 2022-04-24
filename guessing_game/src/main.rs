use std::io;
use std::cmp::Ordering;

use rand::Rng;

enum Food {
    Sushi,
    Kimchi,
    Taco,
    Bbq,
    Fried,
}

fn my_favorite(food: Food) {
    match food {
        Food::Sushi => println!("sushi"),
        Food::Kimchi => println!("kimchi"),
        Food::Taco => println!("taco"),
        Food::Bbq => println!("bbq"),
        Food::Fried => println!("fried")
    }
}

fn main() {
    let mut x = 9;
    println!("I am {} years old.", x);
    let mut x = x + 1;
    let mut x = x * 2;
    println!("Now I am {}.", x);

    let sushi = Food::Sushi;
    my_favorite(sushi);
    // return my_favorite(sushi);
}