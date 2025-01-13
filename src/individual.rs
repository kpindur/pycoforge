use pyo3::prelude::*;

use mycoforge::common::traits::Individual;
use mycoforge::tree::{components::TreeGenotype, core::individual::TreeIndividual};

use crate::tree::PyTreeGenotype;

#[pyclass]
pub struct PyTreeIndividual {
    internal: TreeIndividual<TreeGenotype>
}

#[pymethods]
impl PyTreeIndividual {
    #[new]
    fn new(genotype: &PyTreeGenotype, fitness: f64) -> Self {
        return Self { internal: TreeIndividual::new(genotype.internal.clone(), fitness) };
    }

    fn genotype(&self) -> PyTreeGenotype {
        return PyTreeGenotype { internal: self.internal.genotype().clone() };
    }

    #[staticmethod]
    fn from_vecs(py: Python<'_>, genotypes: PyObject, fitness: Vec<f64>) -> PyResult<Vec<Self>> {
        let genotypes: Vec<PyTreeGenotype> = genotypes.extract(py)?;
        let internal_genotypes = genotypes.iter()
            .map(|g| g.internal.clone())
            .collect::<Vec<TreeGenotype>>();

        return Ok(TreeIndividual::from_vecs(&internal_genotypes, &fitness).into_iter()
            .map(|ind| Self { internal: ind })
            .collect());
    }
}
