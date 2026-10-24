// The rules of the game, with no printing or reading.
// Keeping rules apart from input/output makes them easy to test.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// (highest number, number of tries)
    pub fn settings(self) -> (u32, u32) {
        match self {
            Difficulty::Easy => (50, 10),
            Difficulty::Medium => (100, 7),
            Difficulty::Hard => (1000, 10),
        }
    }

    pub fn from_choice(choice: &str) -> Option<Difficulty> {
        match choice.trim().to_lowercase().as_str() {
            "1" | "e" | "easy" => Some(Difficulty::Easy),
            "2" | "m" | "medium" => Some(Difficulty::Medium),
            "3" | "h" | "hard" => Some(Difficulty::Hard),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Hint {
    TooLow { close: bool },
    TooHigh { close: bool },
    Correct,
}

pub struct Game {
    secret: u32,
    pub max: u32,
    pub tries_left: u32,
    pub tries_used: u32,
}

impl Game {
    pub fn new(secret: u32, difficulty: Difficulty) -> Game {
        let (max, tries) = difficulty.settings();
        Game { secret, max, tries_left: tries, tries_used: 0 }
    }

    /// Scores one guess and uses up one try.
    pub fn guess(&mut self, number: u32) -> Hint {
        self.tries_left -= 1;
        self.tries_used += 1;
        // "close" means within 5% of the range.
        let close = self.secret.abs_diff(number) <= (self.max / 20).max(2);
        if number < self.secret {
            Hint::TooLow { close }
        } else if number > self.secret {
            Hint::TooHigh { close }
        } else {
            Hint::Correct
        }
    }

    pub fn secret(&self) -> u32 {
        self.secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hints() {
        let mut g = Game::new(40, Difficulty::Medium);
        assert_eq!(g.guess(10), Hint::TooLow { close: false });
        assert_eq!(g.guess(43), Hint::TooHigh { close: true });
        assert_eq!(g.guess(40), Hint::Correct);
        assert_eq!(g.tries_used, 3);
        assert_eq!(g.tries_left, 4);
    }

    #[test]
    fn difficulty_choices() {
        assert_eq!(Difficulty::from_choice(" Hard "), Some(Difficulty::Hard));
        assert_eq!(Difficulty::from_choice("2"), Some(Difficulty::Medium));
        assert_eq!(Difficulty::from_choice("insane"), None);
    }
}
