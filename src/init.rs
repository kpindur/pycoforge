use pyo3::prelude::*;

use mycoforge::common::traits::Initializer;
use mycoforge::tree::operators::init::{Grow, Full};

use crate::sampler::PyOperatorSampler;
use crate::tree::PyTreeGenotype;
use crate::random::PyRng;

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
