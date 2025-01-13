use pyo3::prelude::*;

use mycoforge::operators::sampler::OperatorSampler;

#[pyclass]
pub struct PyOperatorSampler {
    pub(crate) internal: OperatorSampler
}

#[pymethods]
impl PyOperatorSampler {
    #[new]
    fn new(operators: Vec<String>, arity: Vec<usize>, weights: Vec<f64>) -> Self {
        return Self { internal: OperatorSampler::new(operators, arity, weights) };
    }

    fn operators(&self) -> Vec<String> { return self.internal.operators().clone(); }

    fn arities(&self) -> Vec<usize> { return self.internal.arities().clone(); }

    fn weights(&self) -> Vec<f64> { return self.internal.weights().clone(); }

    fn update_weights(&mut self, weights: Vec<f64>) -> PyResult<()> {
        self.internal.update_weights(weights);
        return Ok(());
    }
}

