use pyo3::prelude::*;

mod tree;

#[pymodule]
fn pycoforge(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<tree::PyOperatorSampler>()?;
    m.add_class::<tree::PyTreeGenotype>()?;

    return Ok(());
}
