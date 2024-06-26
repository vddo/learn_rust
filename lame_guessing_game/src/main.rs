use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Try to guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input a number as guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
