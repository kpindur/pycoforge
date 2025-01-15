use mycoforge::tree::components::{TreeGenotype, TreeIndividual};
use pyo3::prelude::*;
use rand::RngCore;

use mycoforge::common::traits::Selector;
use mycoforge::tree::operators::select::TournamentSelection;

use crate::individual::PyTreeIndividual;
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
pub struct PyTournamentSelection {
    internal: TournamentSelection
}

#[pymethods]
impl PyTournamentSelection {
    #[new]
    fn new(tournament_size: usize) -> Self {
        return Self { internal: TournamentSelection::new(tournament_size) };
    }

    fn select(&self,
        _py: Python<'_>, py_rng: PyObject, 
        population: Vec<PyTreeIndividual>
    ) -> PyResult<PyTreeGenotype> {
        let mut rng = PyRng { py_rng: &py_rng };

        let population: Vec<TreeIndividual<TreeGenotype>> = population.iter()
            .map(|ind| ind.internal.clone()).collect();
        
        let selected = self.internal.select(&mut rng, &population);

        return Ok(PyTreeGenotype { internal: selected });
    }
}
