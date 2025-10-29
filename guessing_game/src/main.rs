use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number between 1 to 100!");
    let secret_num = rand::rng().random_range(0..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Not a number");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You WIN!!!");
                break;
            }
        }
    }
}
