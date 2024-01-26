use rand::Rng;
use std::{io, thread, time};

fn main() {
    const GUESSES: [&str; 3] = ["rock", "paper", "scissors"];

    println!("Welcome to Rock Paper Scissors");

    loop {
        let enemy = GUESSES[rand::thread_rng().gen_range(0..=2)];

        println!("Enter a choice (rock, paper, scissors)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess == "exit" {
            break
        }

        if !GUESSES.contains(&guess) {
            println!("Guess not valid");
            continue
        }

        let hundred_millis = time::Duration::from_millis(500);   
        println!("Rock!");
        thread::sleep(hundred_millis);
        println!("Paper!");
        thread::sleep(hundred_millis);
        println!("Scissors!");
        thread::sleep(hundred_millis);
        println!("Shoot!");

        if enemy == guess {
            println!("Draw, try again");
        } else if guess == "rock" && enemy == "paper" {
            println!("The enemy chose paper, you loose");
        } else if guess == "paper" && enemy == "scissors" {
            println!("The enemy chose scissors, you loose");
        } else if guess == "scissors" && enemy == "rock" {
            println!("The enemy chose rock, you loose");
        } else {
            println!("The enemy chose {enemy}, you win");
        }
    }
}
