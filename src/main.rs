use clap::Parser;
use crate::cli::GameMode;

mod life;
// mod dots; 
mod cli;


fn main() {
    let args = cli::Args::parse();
    match args.mode {
        GameMode::Dots => todo!(),
        GameMode::Life => life::main::run()
    }
}