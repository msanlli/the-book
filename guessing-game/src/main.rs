use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number or lose your PC!\nPlease input your guess.");

    let secret_number = rand::rng().random_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! Too bad, errasing system..."),
        Ordering::Greater => println!("Too large! Too bad, errasing system..."),
        Ordering::Equal => println!("On point! You keep your system... for now"),
    }
}
