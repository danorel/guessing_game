use std::{io, cmp::Ordering};

mod generator;
mod player;

fn register_number(mut secret_number: &i32) {
    secret_number = generator::random();
}

fn register_players(mut p1: player::Player, mut p2: player::Player) {
    let mut nickname1 = String::new();
    let mut nickname2 = String::new();

    println!("Player 1, enter your nickname:");

    io::stdin()
        .read_line(&mut nickname1)
        .expect("Failed to read the nickname");


    p1 = player::Player{ nickname: String::from(nickname1) };

    println!("Player 2, enter your nickname:");

    io::stdin()
        .read_line(&mut nickname2)
        .expect("Failed to read the nickname");

    p2 = player::Player{ nickname: String::from(nickname2) };
}

fn game(mut p1: player::Player, mut p2: player::Player, secret_number: i32) {
    let mut turn = true;

    loop {
        if turn == true {
            println!("{}, input your number:", p1.nickname);
        }
        

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

fn main() {
    println!("Game 'Guess the number'!");

    let mut p1 = player::Player{ nickname: String::from("") };
    let mut p2  = player::Player{ nickname: String::from("") };

    let mut secret_number: i32 = 0;

    register_number(&secret_number);
    register_players(p1, p2);

    game(p1, p2, secret_number);
}
