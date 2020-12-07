use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
pub fn add(n: i32, m: i32) -> i32 {
    n + m
}


#[pymodule]
pub fn adder(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(add))?;
    Ok(())
}
