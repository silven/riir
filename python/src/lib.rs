#[macro_use] extern crate cpython;
extern crate rayon;

use cpython::{PyResult, PyObject, Python};

py_module_initializer!(librust2py, initlibrust2py, PyInit_librust2py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_as_string", py_fn!(py, sum_as_string_py(a: i64, b:i64)))?;
    m.add(py, "version", py_fn!(py, version_py()))?;
    m.add(py, "sort", py_fn!(py, sort_py(xs: Vec<u64>)))?;

    Ok(())
});

fn version_py(py: Python) -> PyResult<PyObject> {
    let sys = py.import("sys").unwrap();
    let version: String = sys.get(py, "version").unwrap().extract(py).unwrap();

    println!("Python version is {}", version);
    Ok(py.None())
}

fn sum_as_string_py(_: Python, a: i64, b: i64) -> PyResult<String> {
    Ok(format!("{}", a + b).to_string())
}

fn sort_py(_: Python, mut xs: Vec<u64>) -> PyResult<Vec<u64>> {
    xs.sort();
    Ok(xs)
}