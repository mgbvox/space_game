use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum GameMode {
    Dots,
    Life,
}

#[derive(Parser)]
pub struct Args {
    #[arg(value_enum, short = 'm', help = "Choose a game mode")]
    pub mode: GameMode,
}