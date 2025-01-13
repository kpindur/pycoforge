
#![allow(clippy::needless_return)]

use pyo3::prelude::*;

mod sampler;

mod tree;
mod individual;

#[pymodule]
fn pycoforge(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sampler::PyOperatorSampler>()?;
    m.add_class::<tree::PyTreeGenotype>()?;

    return Ok(());
}
