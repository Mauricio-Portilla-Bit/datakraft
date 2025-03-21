use pyo3::prelude::*;
use crate::stats::*;
use crate::fourier::*;

// Exporta el módulo a Python
#[pymodule]
fn datakraft(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(variance, m)?)?;
    m.add_function(wrap_pyfunction!(fft, m)?)?;
    Ok(())
}

// Importa módulos internos
mod stats;
mod fourier;

