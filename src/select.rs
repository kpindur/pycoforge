use pyo3::prelude::*;

use mycoforge::common::traits::Selector;
use mycoforge::tree::operators::select::TournamentSelection;
use mycoforge::tree::components::{TreeGenotype, TreeIndividual};

use crate::individual::PyTreeIndividual;
use crate::tree::PyTreeGenotype;
use crate::random::PyRng;

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
