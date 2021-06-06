use std::process::exit;

use log::{info, LevelFilter};
use structopt::StructOpt;

mod logger;
mod petrick;
mod qmc;
mod term;
mod utils;
use term::Term;

#[derive(Debug, StructOpt)]
#[structopt(name = "rqmc", about = "An program that minimize boolean functions.")]
struct Opt {
    #[structopt(long = "min", help = "minterms in integer form", required = true)]
    minterms: Vec<u32>,

    #[structopt(long = "not-care", help = "not cares in integer form")]
    not_cares: Vec<u32>,

    #[structopt(short, parse(from_occurrences), help = "increase verbosity")]
    verbosity: u8,
}

fn main() {
    let opt = Opt::from_args();
    let level = match opt.verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };
    if let Err(e) = logger::init_logger(level) {
        eprintln!("Failed to init logger, exit.\n{}", e);
        exit(1);
    }

    let minterms: Vec<Term> = opt.minterms.into_iter().map(|num| num.into()).collect();
    let not_cares: Vec<Term> = opt.not_cares.into_iter().map(|num| num.into()).collect();
    let max_length = minterms
        .iter()
        .chain(&not_cares)
        .map(|t| t.term.len())
        .max()
        .unwrap();
    info!("minterms: {}", utils::format_terms(&minterms, max_length));
    info!("not cares: {}", utils::format_terms(&not_cares, max_length));

    let prime_implicants = qmc::find_prime_implicants(&minterms, &not_cares);
    info!(
        "prime implicates: {}",
        utils::format_terms(&prime_implicants, max_length)
    );

    let terms = petrick::find_essential_prime_implicants(&prime_implicants, &minterms);
    println!("Result: {}", utils::format_terms(&terms, max_length));
}
