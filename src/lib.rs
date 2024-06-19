use pyo3::prelude::*;
mod cairo;
use crate::cairo::CairoCamera;

fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    // access this as import manimforge.cairo.CairoRenderer
    let py = parent_module.py();
    let cairo = PyModule::new_bound(py, "cairo")?;
    cairo.add_class::<CairoCamera>()?;
    parent_module.add_submodule(&cairo)?;
    let sys = PyModule::import_bound(py, "sys")?;
    sys.getattr("modules")?.set_item("manimforge.cairo", cairo)?;
    Ok(())
}

/// Parts of manim that need to do heavy lifting are implemented in this
/// library using Rust. See the ``manimforge`` directory
#[pymodule]
fn manimforge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    Ok(())
}
