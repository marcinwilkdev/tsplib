mod tsp_parser;

use tsp_parser::Tsp;

use pyo3::prelude::*;
use pyo3::exceptions;

#[pyfunction]
fn parse_file(filename: String) -> PyResult<Vec<Vec<u32>>> {
    let tsp = Tsp::from_file(&filename);

    match tsp {
        Some(tsp) => Ok(tsp.get_edges()),
        None => Err(exceptions::PyValueError::new_err("Wrong data format"))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn tsplib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    Ok(())
}
