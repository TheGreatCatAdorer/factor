use crate::prime::*;
use crate::State::*;
use std::env::args;
use std::io;

pub mod prime;

#[derive(Debug, Clone, Copy)]
enum State {
    Start,
    PrimeFirst,
    FactorFirst,
    GcdFirst,
    GcdSecond,
    LcmFirst,
    LcmSecond,
    RangeFirst,
    RangeSecond,
}

const DOC: &str = "\
Factor: a CLI tool for finding primes and values relating to them.

Usage: factor [OPTION]...

All output is in one line per option; each number is followed by a space.

Options:
  -p, --prime     Print primes up to and including a number.
  -g, --gcd       Print the greatest common divisor of two numbers.
  -l, --lcm       Print the least common multiple of two numbers.
  -f, --factor    Print all prime factors of a number, sorted from small to large.
  -r, --range     Print primes within an inclusive range of two numbers.
  -h, --help      Print this help document.
";

fn main() -> Result<(), io::Error> {
    let mut input = args();
    input.next();
    let mut state = Start;
    let mut first = 0;
    let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 23];
    for word in input {
        match state {
            Start if word == "-p" || word == "--prime" => state = PrimeFirst,
            Start if word == "-g" || word == "--gcd" => state = GcdFirst,
            Start if word == "-l" || word == "--lcm" => state = LcmFirst,
            Start if word == "-f" || word == "--factor" => state = FactorFirst,
            Start if word == "-r" || word == "--range" => state = RangeFirst,
            Start if word == "-h" || word == "--help" || word == "help" || word == "?" => {
                print!("{DOC}")
            }
            Start => return Err(io::Error::new(io::ErrorKind::InvalidInput, "mode expected")),
            PrimeFirst => {
                let length = word.parse().unwrap();
                if primes[primes.len() - 1] < length {
                    primes = generate_primes(length as u64);
                }
                for &p in &primes {
                    if p > length {
                        break;
                    }
                    print!("{p} ");
                }
                println!("");
                state = Start;
            }
            FactorFirst => {
                let length = word.parse().unwrap();
                if primes[primes.len() - 1] < length {
                    primes = generate_primes(length as u64);
                }
                factor(length as u64, &primes, |f| print!("{f} "));
                println!("");
                state = Start;
            }
            GcdFirst => {
                first = word.parse().unwrap();
                state = GcdSecond;
            }
            LcmFirst => {
                first = word.parse().unwrap();
                state = LcmSecond;
            }
            RangeFirst => {
                first = word.parse().unwrap();
                state = RangeSecond;
            }
            GcdSecond => {
                println!("{} ", gcd(first, word.parse().unwrap()));
                state = Start;
            }
            LcmSecond => {
                println!("{} ", lcm(first, word.parse().unwrap()));
                state = Start;
            }
            RangeSecond => {
                let length = word.parse().unwrap();
                if primes[primes.len() - 1] < length {
                    primes = generate_primes(length as u64);
                }
                for &p in &primes {
                    if p > length {
                        break;
                    } else if p >= first {
                        print!("{p} ");
                    }
                }
                println!("");
                state = Start;
            }
        }
    }
    Ok(())
}
