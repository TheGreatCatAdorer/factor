use clap::{Parser, Subcommand};
use crate::prime::*;
use crate::CliAction::*;

pub mod prime;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: CliAction,
}

#[derive(Subcommand)]
enum CliAction {
    Primes { start: u64, end: u64 },
    Factors { number: u64 },
    Gcd { first: u64, second: u64 },
    Lcm { first: u64, second: u64 },
}

fn main() {
    let Cli { action } = Cli::parse();
    match action {
        Primes { start, end } => {
            for prime in generate_primes(end) {
                if prime >= start {
                    println!("{prime}");
                }
            }
        }
        Factors { number } => {
            for factor in factors(number) {
                println!("{factor}");
            }
        }
        Gcd { first, second } => {
            println!("{}", gcd(first, second));
        }
        Lcm { first, second } => {
            println!("{}", lcm(first, second));
        }
    }
}
