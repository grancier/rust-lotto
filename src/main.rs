use std::io::{self, BufWriter, Write};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use rand::rngs::OsRng;
use rand::{RngCore, SeedableRng, TryRngCore};
use rand_chacha::ChaCha20Rng;
use rust_lotto::{Game, draw};

#[derive(Parser, Debug)]
#[command(
    name = "rust-lotto",
    version,
    about = "Lottery number generator for NJ Cash 5, NJ Pick-6, Mega Millions, and Powerball",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    game: GameCmd,

    /// Number of independent tickets to generate.
    #[arg(short = 'n', long = "tickets", default_value_t = 1, global = true)]
    tickets: u32,

    /// Seed the RNG for deterministic output (testing). When omitted, draws
    /// are sourced from the operating system CSPRNG.
    #[arg(long, global = true)]
    seed: Option<u64>,
}

#[derive(Subcommand, Debug)]
enum GameCmd {
    /// NJ Cash 5 — 5 numbers from 1–45.
    #[command(visible_alias = "5")]
    Cash5,
    /// NJ Pick-6 — 6 numbers from 1–46.
    #[command(visible_alias = "6")]
    Pick6,
    /// Mega Millions — 5 numbers from 1–70 + Mega Ball from 1–24.
    #[command(visible_alias = "m")]
    Megamillions,
    /// Powerball — 5 numbers from 1–69 + Powerball from 1–26.
    #[command(visible_alias = "p")]
    Powerball,
}

impl From<&GameCmd> for Game {
    fn from(cmd: &GameCmd) -> Self {
        match cmd {
            GameCmd::Cash5 => Self::Cash5,
            GameCmd::Pick6 => Self::Pick6,
            GameCmd::Megamillions => Self::MegaMillions,
            GameCmd::Powerball => Self::Powerball,
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let game = Game::from(&cli.game);

    let mut rng: Box<dyn RngCore> = match cli.seed {
        Some(seed) => Box::new(ChaCha20Rng::seed_from_u64(seed)),
        None => Box::new(OsRng.unwrap_err()),
    };

    let stdout = io::stdout().lock();
    let mut out = BufWriter::new(stdout);

    for _ in 0..cli.tickets {
        let ticket = draw(game, rng.as_mut());
        if writeln!(out, "{ticket}").is_err() {
            return ExitCode::FAILURE;
        }
    }

    if out.flush().is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
