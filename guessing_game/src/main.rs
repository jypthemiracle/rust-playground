// use std::io;
// use std::cmp::Ordering;
//
// use rand::Rng;
//
// enum Food {
//     Sushi,
//     Kimchi,
//     Taco,
//     Bbq,
//     Fried,
// }
//
// fn my_favorite(food: Food) {
//     match food {
//         Food::Sushi => println!("sushi"),
//         Food::Kimchi => println!("kimchi"),
//         Food::Taco => println!("taco"),
//         Food::Bbq => println!("bbq"),
//         Food::Fried => println!("fried")
//     }
// }
//
// fn main() {
//     let mut x = 9;
//     println!("I am {} years old.", x);
//     let mut x = x + 1;
//     let mut x = x * 2;
//     println!("Now I am {}.", x);
//
//     let sushi = Food::Sushi;
//     my_favorite(sushi);
//     // return my_favorite(sushi);
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_new(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word_new(&s);

    s.clear(); // Error!

    println!("the first word is: {}", word);
}