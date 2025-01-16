
#![allow(clippy::needless_return)]

use pyo3::prelude::*;

mod random;
mod sampler;

mod tree;
mod individual;

mod init;
mod select;

mod mutation;
mod crossover;

#[pymodule]
fn pycoforge(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sampler::PyOperatorSampler>()?;
    m.add_class::<tree::PyTreeGenotype>()?;

    m.add_class::<init::PyGrow>()?;
    m.add_class::<init::PyFull>()?;

    m.add_class::<mutation::PySubtreeMutation>()?;
    m.add_class::<mutation::PySizeFairMutation>()?;
    m.add_class::<mutation::PyPointMutation>()?;
    m.add_class::<mutation::PyConstantMutation>()?;

    m.add_class::<crossover::PySubtreeCrossover>()?;

    return Ok(());
}
