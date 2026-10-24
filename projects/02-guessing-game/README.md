# Project 2: Number guessing game

Guess the secret number with hints like "Too low" and "A little higher!".
Three difficulty levels, a best score, and play-again. Built in Chapter 2 of
*Rust in 199 Pages*.

```bash
cargo run -p guessing-game                 # play
cargo run -p guessing-game -- --seed 7     # same secret every time
cargo test -p guessing-game                # unit tests + a scripted full game
```

## Layout

```text
02-guessing-game/
├── src/
│   ├── main.rs     the game loop: menus, reading guesses, printing hints
│   ├── game.rs     the rules: difficulty, hints, tries (no input/output)
│   └── random.rs   a tiny seeded random number generator
└── tests/
    └── play.rs     runs the real program with typed input
```

## Ideas to extend

- Save the best score to a file (Chapter 6).
- Add a "custom" difficulty where the player picks the range.
- Let the computer guess *your* number using the halving strategy.
