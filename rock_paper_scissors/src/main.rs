use rand::Rng;
use std::io;

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

        if !GUESSES.contains(&guess) {
            println!("Guess not valid");
            continue
        }

        if enemy == guess {
            println!("Draw, try again");
        } else if guess == "rock" && enemy == "paper" {
            println!("The enemy chose paper, you loose");
        } else if guess == "paper" && enemy == "scissors" {
            println!("The enemy chose scissors, you loose");
        } else if guess == "scissors" && enemy == "rock" {
            println!("The enemy chose rock, you loose");
        } else if guess == "paper" && enemy == "rock" {
            println!("The enemy chose rock, you win");
        } else if guess == "scissors" && enemy == "paper" {
            println!("The enemy chose paper, you wim");
        } else if guess == "rock" && enemy == "scissors" {
            println!("The enemy chose scissors, you win");
        }
    }
}
