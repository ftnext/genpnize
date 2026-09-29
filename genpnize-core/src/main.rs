use clap::Parser;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", genpnize_core::genpnize(&args.input));
}
