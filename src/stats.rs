use pyo3::prelude::*;

#[pyfunction]
pub fn mean(data: Vec<f64>) -> PyResult<f64> {
    let sum: f64 = data.iter().sum();
    Ok(sum / data.len() as f64)
}

#[pyfunction]
pub fn variance(data: Vec<f64>) -> PyResult<f64> {
    let mean = mean(data.clone())?;
    let var: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    Ok(var)
}
