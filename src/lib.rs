mod tsp_parser;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn parse_file(filename: String) -> PyResult<Vec<Vec<u32>>> {
    Ok(vec![vec![]])
}

/// A Python module implemented in Rust.
#[pymodule]
fn tsplib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
