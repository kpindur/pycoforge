use std::collections::HashMap;

use pyo3::prelude::*;
use mycoforge::tree::core::tree::TreeGenotype;
use crate::sampler::PyOperatorSampler;

#[pyclass]
pub struct PyTreeGenotype {
    internal: TreeGenotype
}

#[pymethods]
impl PyTreeGenotype {
    #[new]
    fn new(arena: Vec<String>, children: HashMap<usize, Vec<usize>>) -> Self {
        return Self { internal: TreeGenotype::new(arena, children) };
    }

    #[staticmethod]
    fn with_arena(arena: Vec<String>) -> Self {
        return Self { internal: TreeGenotype::with_arena(arena) };
    }

    fn __str__(&self) -> String {
        return self.internal.to_string();
    }

    fn arena(&self) -> Vec<String> {
        return self.internal.arena().clone();
    }

    fn children(&self) -> HashMap<usize, Vec<usize>> {
        return self.internal.children().clone();
    }

    fn subtree(&self, root: usize) -> usize {
        return self.internal.subtree(root);
    }

    fn construct_children(&mut self, sampler: &PyOperatorSampler) -> PyResult<()> {
        *self.internal.children_mut() = self.internal.construct_children(&sampler.internal);
        return Ok(());
    }
}
