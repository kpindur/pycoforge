use pyo3::prelude::*;
use rand::RngCore;

use mycoforge::common::traits::Initializer;
use mycoforge::tree::operators::init::{Grow, Full};

use crate::sampler::PyOperatorSampler;
use crate::tree::PyTreeGenotype;

struct PyRng<'a> {
    py_rng: &'a PyObject
}

impl RngCore for PyRng<'_> {
    fn next_u32(&mut self) -> u32 {
        return Python::with_gil(|py| {
            self.py_rng.call_method1(py, "randint", (0_u32, u32::MAX))
                .unwrap()
                .extract(py)
                .unwrap()
        });
    }

    fn next_u64(&mut self) -> u64 {
        return Python::with_gil(|py| {
            self.py_rng.call_method1(py, "randint", (0_u64, u64::MAX))
                .unwrap()
                .extract(py)
                .unwrap()
        });
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut i = 0;
        while i < dest.len() {
            let random_bytes = self.next_u64().to_le_bytes();
            let remaining = dest.len() - i;
            let to_copy = std::cmp::min(remaining, 8);
            dest[i..i+to_copy].copy_from_slice(&random_bytes[..to_copy]);
            i += to_copy;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[pyclass]
pub struct PyGrow {
    internal: Grow
}

#[pymethods]
impl PyGrow {
    #[new]
    fn new(min_depth: usize, max_depth: usize) -> Self {
        return PyGrow { internal: Grow::new(min_depth, max_depth) };
    }

    fn min_depth(&self) -> usize {
        return self.internal.min_depth();
    }

    fn max_depth(&self) -> usize {
        return self.internal.max_depth();
    }

    fn initialize(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        sampler: &PyOperatorSampler
    ) -> PyTreeGenotype {
        let mut rng = PyRng { py_rng: &py_rng };

        return PyTreeGenotype { internal: self.internal.initialize(&mut rng, &sampler.internal) };
    }
}

#[pyclass]
pub struct PyFull {
    internal: Full
}

#[pymethods]
impl PyFull {
    #[new]
    fn new(depth: usize) -> Self {
        return PyFull { internal: Full::new(depth) };
    }

    fn depth(&self) -> usize {
        return self.internal.depth();
    }

    fn initialize(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        sampler: &PyOperatorSampler
    ) -> PyTreeGenotype {
        let mut rng = PyRng { py_rng: &py_rng };

        return PyTreeGenotype { internal: self.internal.initialize(&mut rng, &sampler.internal) };
    }

}
