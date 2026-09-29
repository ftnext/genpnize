use clap::Parser;
use std::io::Read;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let input = if args.input == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        args.input
    };
    println!("{}", genpnize_core::genpnize(input.trim_end()));
    Ok(())
}
