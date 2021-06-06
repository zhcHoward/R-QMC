use structopt::StructOpt;

mod petrick;
mod qmc;
mod term;
mod utils;
use term::Term;

#[derive(Debug, StructOpt)]
#[structopt(name = "rqmc", about = "An program that minimize boolean functions.")]
struct Opt {
    #[structopt(long = "min")]
    minterms: Vec<u32>,

    #[structopt(long = "not-care")]
    not_cares: Vec<u32>,
}

fn main() {
    let opt = Opt::from_args();
    let minterms: Vec<Term> = opt.minterms.into_iter().map(|num| num.into()).collect();
    let not_cares: Vec<Term> = opt.not_cares.into_iter().map(|num| num.into()).collect();

    let prime_implicants = qmc::find_prime_implicants(&minterms, &not_cares);

    for p in &prime_implicants {
        println!("{}", p);
    }
    println!("");

    let terms = petrick::find_essential_prime_implicants(&prime_implicants, &minterms);
    let max_length = terms.iter().map(|t| t.term.len()).max().unwrap();
    for p in terms.iter() {
        println!("{:0>1$}", p, max_length);
    }
}
