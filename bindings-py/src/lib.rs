use clap::Parser;
use clap_stdin::FileOrStdin;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[derive(Parser)]
struct Cli {
    input: FileOrStdin,
}

#[pyfunction]
fn main(py: Python<'_>) -> PyResult<()> {
    let argv: Vec<String> = py.import("sys")?.getattr("argv")?.extract()?;
    let args = Cli::parse_from(argv);
    let input = args
        .input
        .contents()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    println!(
        "{}",
        genpnize_core::genpnize(&input.replace(['\r', '\n'], ""))
    );
    Ok(())
}

#[pymodule]
mod genpnize {
    #[pymodule_export]
    use super::main;
}
