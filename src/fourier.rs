use pyo3::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};

#[pyfunction]
pub fn fft(input: Vec<f64>) -> PyResult<Vec<(f64, f64)>> {
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(input.len());

    let mut buffer: Vec<Complex<f64>> = input.into_iter().map(|x| Complex::new(x, 0.0)).collect();
    fft.process(&mut buffer);

    let result = buffer.into_iter().map(|c| (c.re, c.im)).collect();
    Ok(result)
}
