use std::collections::HashMap;

use pyo3::prelude::*;
use mycoforge::{operators::sampler::OperatorSampler, tree::core::tree::TreeGenotype};

#[pyclass]
pub struct PyOperatorSampler {
    internal: OperatorSampler
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
