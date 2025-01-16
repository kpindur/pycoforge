use pyo3::prelude::*;

use mycoforge::common::traits::Mutator;
use mycoforge::tree::operators::mutation::{SubtreeMutation, PointMutation, ConstantMutation, SizeFairMutation};

use crate::{sampler::PyOperatorSampler, tree::PyTreeGenotype};
use crate::random::PyRng;

#[pyclass]
pub struct PySubtreeMutation {
    pub(crate) internal: SubtreeMutation
}

#[pymethods]
impl PySubtreeMutation {
    #[new]
    fn new(probability: f64, depth_limits: (usize, usize)) -> PyResult<Self> {
        return Ok(Self {
            internal: SubtreeMutation::new(probability, depth_limits)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        });
    }

    #[staticmethod]
    fn default() -> Self {
        return Self { internal: SubtreeMutation::default() };
    }

    fn probability(&self) -> f64 {
        return self.internal.probability();
    }

    fn depth_limits(&self) -> (usize, usize) {
        return self.internal.depth_limits();
    }

    fn variate(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        individual: &PyTreeGenotype,
        sampler: &PyOperatorSampler
    ) -> PyTreeGenotype {
        let mut rng = PyRng { py_rng: &py_rng };
        let child = self.internal.variate(
            &mut rng, 
            &individual.internal,
            &sampler.internal
        );

        return PyTreeGenotype { internal: child };
    }
}

#[pyclass]
pub struct PySizeFairMutation {
    pub(crate) internal: SizeFairMutation
}

#[pymethods]
impl PySizeFairMutation {
    #[new]
    fn new(probability: f64, dynamic_limit: bool) -> PyResult<Self> {
        return Ok(Self {
            internal: SizeFairMutation::new(probability, dynamic_limit)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        });
    }

    #[staticmethod]
    fn default() -> Self {
        return Self { internal: SizeFairMutation::default() };
    }

    fn probability(&self) -> f64 {
        return self.internal.probability();
    }

    fn variate(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        individual: &PyTreeGenotype,
        sampler: &PyOperatorSampler
    ) -> PyTreeGenotype {
        let mut rng = PyRng { py_rng: &py_rng };
        let child = self.internal.variate(
            &mut rng, 
            &individual.internal,
            &sampler.internal
        );

        return PyTreeGenotype { internal: child };
    }
}

#[pyclass]
pub struct PyPointMutation {
    pub(crate) internal: PointMutation
}

#[pymethods]
impl PyPointMutation {
    #[new]
    fn new(probability: f64) -> PyResult<Self> {
        return Ok(Self {
            internal: PointMutation::new(probability)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        });
    }

    #[staticmethod]
    fn default() -> Self {
        return Self { internal: PointMutation::default() };
    }

    fn probability(&self) -> f64 {
        return self.internal.probability();
    }

    fn variate(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        individual: &PyTreeGenotype,
        sampler: &PyOperatorSampler
    ) -> PyTreeGenotype {
        let mut rng = PyRng { py_rng: &py_rng };
        let child = self.internal.variate(
            &mut rng, 
            &individual.internal,
            &sampler.internal
        );

        return PyTreeGenotype { internal: child };
    }
}

#[pyclass]
pub struct PyConstantMutation {
    pub(crate) internal: ConstantMutation
}

#[pymethods]
impl PyConstantMutation {
    #[new]
    #[pyo3(signature = (probability, mutation_rate, range_limits=None))]
    fn new(probability: f64, mutation_rate: f64, range_limits: Option<(f64, f64)>) -> PyResult<Self> {
        return Ok(Self {
            internal: ConstantMutation::new(probability, mutation_rate, range_limits)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        });
    }

    #[staticmethod]
    fn default() -> Self {
        return Self { internal: ConstantMutation::default() };
    }

    fn probability(&self) -> f64 {
        return self.internal.probability();
    }

    fn variate(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        individual: &PyTreeGenotype,
        sampler: &PyOperatorSampler
    ) -> PyTreeGenotype {
        let mut rng = PyRng { py_rng: &py_rng };
        let child = self.internal.variate(
            &mut rng, 
            &individual.internal,
            &sampler.internal
        );

        return PyTreeGenotype { internal: child };
    }
}
