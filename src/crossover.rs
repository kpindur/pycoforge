use pyo3::prelude::*;

use mycoforge::common::traits::Crossoverer;
use mycoforge::tree::operators::crossover::SubtreeCrossover;

use crate::{sampler::PyOperatorSampler, tree::PyTreeGenotype};
use crate::random::PyRng;

/// Python wrapper for SubtreeCrossover operator
#[pyclass(name = "SubtreeCrossover")]
pub struct PySubtreeCrossover {
    pub(crate) internal: SubtreeCrossover
}

#[pymethods]
impl PySubtreeCrossover {
    #[new]
    fn new(probability: f64) -> PyResult<Self> {
        return Ok(Self {
            internal: SubtreeCrossover::new(probability)
                .map_err(|e| 
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        });
    }

    #[staticmethod]
    fn default() -> Self {
        return Self { internal: SubtreeCrossover::default() };
    }

    #[getter]
    /// The probability of applying crossover
    fn probability(&self) -> f64 {
        return self.internal.probability();
    }

    /// Create a string representation of the crossover operator
    fn __repr__(&self) -> String {
        return format!("SubtreeCrossover(probability={})", self.probability());
    }

    /// Create a string representation of the crossover operator
    fn __str__(&self) -> String {
        return self.__repr__();
    }

    /// Compare two crossover operators for equality
    fn __eq__(&self, other: &PySubtreeCrossover) -> bool {
        return self.probability() == other.probability();
    }

    /// Create a copy of the crossover operator
    fn __copy__(&self) -> PySubtreeCrossover {
        let probability = self.internal.probability();

        return PySubtreeCrossover::new(probability).expect("Failed to copy SubtreeCrossover!");
    }

    fn variate(&self, 
        _py: Python<'_>, py_rng: PyObject, 
        parent1: &PyTreeGenotype, parent2: &PyTreeGenotype,
        sampler: &PyOperatorSampler
    ) -> Vec<PyTreeGenotype> {
        let mut rng = PyRng { py_rng: &py_rng };
        let children = self.internal.variate(
            &mut rng, 
            &parent1.internal, &parent2.internal,
            &sampler.internal
        );

        return children.into_iter()
            .map(|child| PyTreeGenotype { internal: child })
            .collect();
    }
}
