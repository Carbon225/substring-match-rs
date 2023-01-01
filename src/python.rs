use pyo3::prelude::*;

#[pyclass]
struct SubstringMatcher(super::SubstringMatcher);

#[pymethods]
impl SubstringMatcher {
    #[new]
    fn new(texts: Vec<&str>) -> Self {
        Self(super::SubstringMatcher::new(texts.into_iter()))
    }

    fn find(&self, pattern: &str) -> Vec<&str> {
        self.0.find(pattern).collect()
    }
}

#[pymodule]
fn substring_match(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SubstringMatcher>()?;
    Ok(())
}
