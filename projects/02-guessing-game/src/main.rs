// Project 2: Number guessing game
//
//   cargo run -p guessing-game
//   cargo run -p guessing-game -- --seed 7     (same secret every time)

mod game;
mod random;

use game::{Difficulty, Game, Hint};
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut rng = match args.iter().position(|a| a == "--seed") {
        Some(i) => {
            let seed = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1);
            random::Rng::from_seed(seed)
        }
        None => random::Rng::from_time(),
    };

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut best: Option<u32> = None;

    println!("=== Guess the Number ===");
    'games: loop {
        // Choose a difficulty; keep asking until the answer is valid.
        let difficulty = loop {
            prompt("Difficulty? 1) easy 1-50  2) medium 1-100  3) hard 1-1000: ");
            let Some(Ok(line)) = lines.next() else { break 'games };
            match Difficulty::from_choice(&line) {
                Some(d) => break d,
                None => println!("Please type 1, 2 or 3."),
            }
        };

        let (max, _) = difficulty.settings();
        let mut game = Game::new(rng.between(1, max), difficulty);
        println!("I'm thinking of a number from 1 to {max}. You have {} tries.", game.tries_left);

        // One round: read guesses until correct or out of tries.
        while game.tries_left > 0 {
            prompt(&format!("[{} left] Your guess: ", game.tries_left));
            let Some(Ok(line)) = lines.next() else { break 'games };
            let number: u32 = match line.trim().parse() {
                Ok(n) if (1..=max).contains(&n) => n,
                Ok(_) => {
                    println!("Stay between 1 and {max}.");
                    continue;
                }
                Err(_) => {
                    println!("That's not a whole number.");
                    continue;
                }
            };
            match game.guess(number) {
                Hint::TooLow { close: true } => println!("A little higher!"),
                Hint::TooLow { close: false } => println!("Too low."),
                Hint::TooHigh { close: true } => println!("A little lower!"),
                Hint::TooHigh { close: false } => println!("Too high."),
                Hint::Correct => {
                    println!("Correct! You got it in {} tries.", game.tries_used);
                    if best.is_none_or(|b| game.tries_used < b) {
                        best = Some(game.tries_used);
                        println!("New best score!");
                    }
                    break;
                }
            }
        }
        if game.tries_left == 0 && game.tries_used > 0 {
            println!("Out of tries. The number was {}.", game.secret());
        }

        prompt("Play again? (y/n): ");
        match lines.next() {
            Some(Ok(answer)) if answer.trim().eq_ignore_ascii_case("y") => continue,
            _ => break,
        }
    }

    if let Some(b) = best {
        println!("Best score this session: {b} tries");
    }
    println!("Thanks for playing!");
}

fn prompt(text: &str) {
    print!("{text}");
    io::stdout().flush().expect("could not flush stdout");
}
