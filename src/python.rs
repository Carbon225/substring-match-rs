use pyo3::{prelude::*, types::PyList};

#[pyclass]
struct SubstringMatcher(super::SubstringMatcher);

#[pymethods]
impl SubstringMatcher {
    #[new]
    fn new(texts: Py<PyList>) -> Self {
        // have to lock?
        Python::with_gil(|py| {
            let texts = texts.into_ref(py).into_iter().map(|s| s.extract().unwrap());
            let inner = super::SubstringMatcher::new(texts);
            Self(inner)
        })
    }

    fn find(&self, pattern: &str) -> Vec<String> {
        self.0.find(pattern).into_iter().map(|s| s.to_string()).collect()
    }
}

#[pymodule]
fn substring_match(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SubstringMatcher>()?;
    Ok(())
}
