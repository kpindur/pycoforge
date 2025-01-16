use pyo3::prelude::*;
use rand::RngCore;

use mycoforge::common::traits::Crossoverer;
use mycoforge::tree::operators::crossover::SubtreeCrossover;

use crate::{sampler::PyOperatorSampler, tree::PyTreeGenotype};

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
        self.internal.probability()
    }

    /// Create a string representation of the crossover operator
    fn __repr__(&self) -> String {
        format!("SubtreeCrossover(probability={})", self.probability())
    }

    /// Create a string representation of the crossover operator
    fn __str__(&self) -> String {
        self.__repr__()
    }

    /// Compare two crossover operators for equality
    fn __eq__(&self, other: &PySubtreeCrossover) -> bool {
        self.probability() == other.probability()
    }

    /// Create a copy of the crossover operator
    fn __copy__(&self) -> PySubtreeCrossover {
        PySubtreeCrossover {
            internal: self.internal.clone()
        }
    }
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
