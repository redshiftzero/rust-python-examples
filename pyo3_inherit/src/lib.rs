use pyo3::class::basic::PyObjectProtocol;
use pyo3::prelude::*;


#[pyclass(subclass)]
pub struct Animal {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    age: u8,
    hours_since_last_fed: u8,
}

#[pymethods]
impl Animal {
    #[new]
    fn new(name: String, age: u8, hours_since_last_fed: u8) -> Self {
        Animal{ name, age, hours_since_last_fed }
    }

    fn feed(&mut self) {
        self.hours_since_last_fed = 0;
    }
}

#[pyclass(extends=Animal)]
pub struct Lion {
    #[pyo3(get)]
    favorite_meat: String,
}

#[pymethods]
impl Lion {
    #[new]
    fn new(name: String, age: u8, hours_since_last_fed: u8, favorite_meat: String) -> PyResult<(Self, Animal)> {
        Ok((Lion{ favorite_meat }, Animal{ name, age, hours_since_last_fed }))
    }

    fn roar(&self) -> String {
        "ROAR!!!!".to_string()
    }
}


#[pyproto]
impl PyObjectProtocol for Animal {
    fn __str__(&self) -> PyResult<String> {
        Ok(String::from(format!(
            "Animal: {}",
            self.name,
        )))
    }
}


#[pymodule]
pub fn zoo(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<Animal>()?;
    module.add_class::<Lion>()?;
    Ok(())
}
