use numpy::ndarray::{s, Array2, ArrayView1, ArrayView2, Axis};
use numpy::PyReadonlyArray2;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict};
use std::iter;

/// Check if two points are approximately equal
/// 
/// The x and y values of the points are scaled by the relative tolerance and
/// compared to the absolute tolerance. If the difference between the two
/// points is less than or equal to the absolute tolerance, then the points are
/// considered equal.
/// 
/// ## Parameters
/// - `p1`: The first point (x, y)
/// - `p2`: The second point (x, y)
/// 
/// ## Returns
/// - `bool`: True if the points are approximately equal, False otherwise
fn consider_points_equal_2d(
    p1: ArrayView1<f64>,
    p2: ArrayView1<f64>,
) -> bool {
    // relative tolerance
    let rtol: f64 = 1e-5;
    // absolute tolerance
    let atol: f64 = 1e-6; // TODO make this based off vmobject
    // Use a logical AND instead of a bitwise AND here to encourage
    // short-circuiting -- That is, don't evaluate the second condition if the
    // first one is false.
    ( (p1[0] - p2[0]).abs() <= atol + rtol * p1[0].abs() ) &&
    ( (p1[1] - p2[1]).abs() <= atol + rtol * p1[1].abs() )
    // Is there a difference between:
    // - bool & bool
    // - !bool & !bool
    // - bool && bool
}

/// This function creates subpaths by splitting the input points into segments
/// based on certain conditions.
/// 
/// It uses nppcc (number of points per cubic curve, set to 4) and attempts to
/// find points that should not be considered part of the same segment based on
/// the consider_points_equal_2d function.
/// 
/// The function filters indices based on this condition and keeps segments
/// where there is a sufficient number of points (nppcc) between splits.
/// 
/// Handles all points in the array, but only keeps segments where there are at
/// least four points between each split.
/// 
/// ## Parameters
/// 
/// - `points`: A 2D array of points `((x1, y1), (x2, y2), ...)`
/// 
/// ## Returns
/// 
/// - `Vec<Array2<f64>>`: A vector of 2D arrays of points, each representing a
///  subpath
fn gen_subpaths_from_points_2d(
    points: ArrayView2<f64>,
) -> Vec<Array2<f64>> {
    // Number of Points Per Cubic Curve, or
    // Number of Points Per Continuous Curve?
    let nppcc = 4;
    let filtered = (nppcc..points.len_of(Axis(0)))
        .step_by(nppcc).filter(|&n| {
            !consider_points_equal_2d(
            points.index_axis(Axis(0), n - 1),
            points.index_axis(Axis(0), n),
        )});
    let split_indicies: Vec<usize> = iter::once(0)
        .chain(filtered)
        .chain(iter::once(points.len_of(Axis(0))))
        .collect();

    split_indicies
        .iter()
        .zip(split_indicies.iter().skip(1))
        .filter_map(|(&i1, &i2)| {
            if i2 - i1 >= nppcc {
                let path = points
                    .slice(s![i1..i2, ..])
                    .to_owned();
                return Some(path);
            }
            None
        })
        .collect()
}

/// This function segments the points strictly into groups of four (assuming
/// each group represents a cubic Bézier curve segment).
/// 
/// This function ignores point equality conditions and simply slices the
/// points array into contiguous blocks of four points (nppcc).
/// 
/// It adjusts the length of points to be a multiple of 4 by removing any
/// remainder points that don’t complete a group of four.
/// 
/// Trims the array to a multiple of four, potentially discarding points that
/// would not form a complete Bézier segment.
/// 
/// ## Parameters
/// 
/// - `points`: A 2D array of points `((x1, y1), (x2, y2), (x3, y3), (x4, y4))`
/// 
/// ## Returns
/// 
/// - `Vec<Array2<f64>>`: A vector of 2D arrays of points, each representing a
/// cubic Bézier curve segment
fn gen_cubic_bezier_tuples_from_points(
    points: ArrayView2<f64>
) -> Vec<Array2<f64>> {
    // Number of Points Per Cubic Curve, or
    // Number of Points Per Continuous Curve?
    let nppcc = 4;
    let remainder = points.len() % nppcc;
    let points = points.slice(s![..points.len_of(Axis(0)) - remainder, ..]);
    (0..points.len_of(Axis(0)))
        .step_by(nppcc)
        .map(|i| points.slice(s![i..i + nppcc, ..]).to_owned())
        .collect()
}

/// The base class for Manim.Camera with --renderer=cairo
#[pyclass(subclass)]
#[derive(Clone)]
pub struct CairoCamera;

#[pymethods]
impl CairoCamera {
    #[new]
    #[pyo3(signature=(*_args, **_kwargs))]
    fn new(
        _args: &Bound<'_, PyTuple>,
        _kwargs: Option<&Bound<'_, PyDict>>
    ) -> Self {
        Self
    }

    pub fn set_cairo_context_path<'py>(
        &self,
        py: Python<'py>,
        ctx: Py<PyAny>,
        vmobject: Py<PyAny>,
        // old_points: PyReadonlyArray2<'py, f64>,
    ) -> PyResult<()> {
        // vmobject.points is the same type as `points`
        let vm_points = vmobject.getattr(py, "points")?;
        let vm_readonly_array = vm_points.extract::<PyReadonlyArray2<f64>>(py)?;
        let points = vm_readonly_array.as_array();
        // let old_points = points.as_array();
        // assert_eq!(points, old_points);
        // We assume context is correct because serializing it into the Rust binding cairo::Context is too much work
        ctx.call_method0(py, intern!(py, "new_path"))?;
        let subpaths = gen_subpaths_from_points_2d(points);
        for subpath in subpaths {
            let quads = gen_cubic_bezier_tuples_from_points(subpath.view());
            ctx.call_method0(py, intern!(py, "new_sub_path"))?;
            let start = subpath.index_axis(Axis(0), 0);
            ctx.call_method1(py, intern!(py, "move_to"), (start[0], start[1]))?;
            for bezier_tuples in quads {
                let _p0 = bezier_tuples.index_axis(Axis(0), 0);
                let p1 = bezier_tuples.index_axis(Axis(0), 1);
                let p2 = bezier_tuples.index_axis(Axis(0), 2);
                let p3 = bezier_tuples.index_axis(Axis(0), 3);
                ctx.call_method1(
                    py,
                    intern!(py, "curve_to"),
                    (p1[0], p1[1], p2[0], p2[1], p3[0], p3[1]),
                )?;
            }
            if consider_points_equal_2d(
                subpath.index_axis(Axis(0), 0),
                subpath.index_axis(Axis(0), subpath.len_of(Axis(0)) - 1),
            ) {
                ctx.call_method0(py, intern!(py, "close_path"))?;
            }

        }
        Ok(())
    }

    pub fn __copy__(&self) -> Self {
        self.clone()
    }

    pub fn __deepcopy__(&self, _memo: Py<PyAny>) -> Self {
        self.clone()
    }
}
