mod cli;
mod errors;
mod models;
mod utils;
mod years;

use clap::Parser;
use cli::Args;
use utils::measure::run_timed;

fn main() {
    let args = Args::parse();
    let year = args.year.parse::<u16>().unwrap();
    let day = args.day.parse::<u8>().unwrap();

    run_timed(args.time, || {
        if let Err(e) = years::dispatch(year, day) {
            eprintln!("Error: {}", e);
        }
    });
}
