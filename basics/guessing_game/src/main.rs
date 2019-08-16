use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    const MIN: u32 = 0;
    const MAX: u32 = 3;

    let number = rand::thread_rng().gen_range(MIN, MAX);

    println!("Please input your guess between {} and {}", MIN, MAX);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("wrong input!");
                continue;
            }
        };
        let t: String = 3;

        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}