// A tiny pseudo-random number generator (xorshift64).
//
// Real programs use the `rand` crate. Writing our own keeps the project
// free of dependencies and lets tests use a fixed seed, so the "random"
// secret number is the same on every test run.

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng {
    state: u64,
}

impl Rng {
    /// Same seed, same sequence of numbers. Useful for tests.
    pub fn from_seed(seed: u64) -> Rng {
        // xorshift must never hold 0, or it outputs 0 forever.
        Rng { state: if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed } }
    }

    /// Seeded from the clock, so each game is different.
    pub fn from_time() -> Rng {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1);
        Rng::from_seed(nanos)
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// A number from `low` to `high`, both included.
    pub fn between(&mut self, low: u32, high: u32) -> u32 {
        let span = (high - low + 1) as u64;
        low + (self.next() % span) as u32
    }
}
