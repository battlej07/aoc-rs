use clap::Parser;

mod cli;

fn main() {
    let aoc = cli::Cli::parse();

    if let Some(command) = aoc.command {
        let res = match command {
            cli::Commands::Run { year, day, part } => aoc_rs::run(year, day, part),
        };

        match res {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
