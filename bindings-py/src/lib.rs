use clap::Parser;
use pyo3::prelude::*;
use std::io::Read;

#[derive(Parser)]
struct Cli {
    input: String,
}

#[pyfunction]
fn main(py: Python<'_>) -> PyResult<()> {
    let argv: Vec<String> = py.import("sys")?.getattr("argv")?.extract()?;
    let args = Cli::parse_from(argv);
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

#[pymodule]
mod genpnize {
    #[pymodule_export]
    use super::main;
}
