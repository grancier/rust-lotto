//! Lottery number generation for NJ Cash 5, NJ Pick-6, Mega Millions, and Powerball.
//!
//! Each ticket is drawn without replacement from the official number pools. For
//! Mega Millions and Powerball, the special ball is drawn from a separate pool,
//! so it may legally equal one of the main numbers.

use std::fmt;

use rand::Rng;
use rand::seq::index::sample;

/// A lottery game supported by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Game {
    /// NJ Cash 5: 5 numbers from 1–45.
    Cash5,
    /// NJ Pick-6: 6 numbers from 1–46.
    Pick6,
    /// Mega Millions: 5 numbers from 1–70 plus a Mega Ball from 1–24.
    MegaMillions,
    /// Powerball: 5 numbers from 1–69 plus a Powerball from 1–26.
    Powerball,
}

#[derive(Debug, Clone, Copy)]
struct Spec {
    main_count: usize,
    main_max: usize,
    special_max: Option<u32>,
}

impl Game {
    const fn spec(self) -> Spec {
        match self {
            Self::Cash5 => Spec {
                main_count: 5,
                main_max: 45,
                special_max: None,
            },
            Self::Pick6 => Spec {
                main_count: 6,
                main_max: 46,
                special_max: None,
            },
            Self::MegaMillions => Spec {
                main_count: 5,
                main_max: 70,
                special_max: Some(24),
            },
            Self::Powerball => Spec {
                main_count: 5,
                main_max: 69,
                special_max: Some(26),
            },
        }
    }

    /// Human-readable game name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Cash5 => "NJ Cash 5",
            Self::Pick6 => "NJ Pick-6",
            Self::MegaMillions => "Mega Millions",
            Self::Powerball => "Powerball",
        }
    }
}

/// A single generated ticket: main numbers plus an optional special ball.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub game: Game,
    pub main: Vec<u32>,
    pub special: Option<u32>,
}

/// Draw one ticket for `game` from `rng`, sampling without replacement.
///
/// Main numbers are unique within the ticket and returned sorted ascending.
/// The special ball (when present) is drawn from a separate pool and may
/// coincide with a main number, matching the official lottery rules.
///
/// # Panics
///
/// Cannot panic in practice: every supported game's pool maximum fits in a
/// `u32`, so the index-to-number conversion always succeeds.
pub fn draw<R: Rng + ?Sized>(game: Game, rng: &mut R) -> Ticket {
    let spec = game.spec();

    let mut main: Vec<u32> = sample(rng, spec.main_max, spec.main_count)
        .iter()
        .map(|i| u32::try_from(i + 1).expect("lottery number fits in u32"))
        .collect();
    main.sort_unstable();

    let special = spec.special_max.map(|max| rng.random_range(1..=max));

    Ticket {
        game,
        main,
        special,
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for n in &self.main {
            if !first {
                f.write_str(" ")?;
            }
            write!(f, "{n:02}")?;
            first = false;
        }
        if let Some(s) = self.special {
            write!(f, "  {s:02}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    use super::*;

    fn all_games() -> [Game; 4] {
        [
            Game::Cash5,
            Game::Pick6,
            Game::MegaMillions,
            Game::Powerball,
        ]
    }

    #[test]
    fn cash5_shape() {
        let mut rng = ChaCha20Rng::seed_from_u64(0);
        let t = draw(Game::Cash5, &mut rng);
        assert_eq!(t.game, Game::Cash5);
        assert_eq!(t.main.len(), 5);
        assert!(t.special.is_none());
        assert!(t.main.iter().all(|n| (1..=45).contains(n)));
    }

    #[test]
    fn pick6_shape() {
        let mut rng = ChaCha20Rng::seed_from_u64(1);
        let t = draw(Game::Pick6, &mut rng);
        assert_eq!(t.main.len(), 6);
        assert!(t.special.is_none());
        assert!(t.main.iter().all(|n| (1..=46).contains(n)));
    }

    #[test]
    fn megamillions_shape() {
        let mut rng = ChaCha20Rng::seed_from_u64(2);
        let t = draw(Game::MegaMillions, &mut rng);
        assert_eq!(t.main.len(), 5);
        assert!(t.main.iter().all(|n| (1..=70).contains(n)));
        let s = t.special.expect("MegaMillions has a Mega Ball");
        assert!((1..=24).contains(&s));
    }

    #[test]
    fn powerball_shape() {
        let mut rng = ChaCha20Rng::seed_from_u64(3);
        let t = draw(Game::Powerball, &mut rng);
        assert_eq!(t.main.len(), 5);
        assert!(t.main.iter().all(|n| (1..=69).contains(n)));
        let s = t.special.expect("Powerball has a Powerball");
        assert!((1..=26).contains(&s));
    }

    #[test]
    fn main_numbers_unique_and_sorted() {
        let mut rng = ChaCha20Rng::seed_from_u64(42);
        for game in all_games() {
            for _ in 0..1000 {
                let t = draw(game, &mut rng);
                let unique: HashSet<_> = t.main.iter().copied().collect();
                assert_eq!(
                    unique.len(),
                    t.main.len(),
                    "duplicate main number in {game:?}"
                );
                assert!(
                    t.main.windows(2).all(|w| w[0] < w[1]),
                    "main not strictly increasing in {game:?}: {:?}",
                    t.main,
                );
            }
        }
    }

    #[test]
    fn special_pool_is_independent() {
        // The Mega Ball / Powerball is sampled from its own pool, so it can
        // legally equal one of the main numbers. Across many draws we expect
        // at least one collision.
        let mut rng = ChaCha20Rng::seed_from_u64(7);
        let mut seen_collision = false;
        for _ in 0..2000 {
            let t = draw(Game::MegaMillions, &mut rng);
            if let Some(s) = t.special {
                if t.main.contains(&s) {
                    seen_collision = true;
                    break;
                }
            }
        }
        assert!(
            seen_collision,
            "special ball never coincided with a main number"
        );
    }

    #[test]
    fn deterministic_for_fixed_seed() {
        let a = {
            let mut rng = ChaCha20Rng::seed_from_u64(0xDEAD_BEEF);
            draw(Game::Powerball, &mut rng)
        };
        let b = {
            let mut rng = ChaCha20Rng::seed_from_u64(0xDEAD_BEEF);
            draw(Game::Powerball, &mut rng)
        };
        assert_eq!(a, b);
    }

    #[test]
    fn display_format() {
        let t = Ticket {
            game: Game::MegaMillions,
            main: vec![3, 17, 24, 31, 58],
            special: Some(9),
        };
        assert_eq!(t.to_string(), "03 17 24 31 58  09");

        let t = Ticket {
            game: Game::Cash5,
            main: vec![1, 2, 3, 4, 5],
            special: None,
        };
        assert_eq!(t.to_string(), "01 02 03 04 05");
    }
}
