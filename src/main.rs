use std::io;
use std::cmp::Ordering;
use std::ops::Range;
use rand::Rng;

fn random () -> i32 {
    rand::thread_rng().gen_range::<i32, Range::<i32>>(1..101)
}

fn main() {
    println!("Game 'Guess the number'!");

    let secret_number = random();

    loop {
        println!("Please, input your number:");

        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read the guessing number!");

        let guess: i32 = match prompt.trim().parse() {
            Ok(num) => num,
            Err(err) => panic!("{err}"),
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }
    }
}
