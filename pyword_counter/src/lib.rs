use std::collections::HashMap;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn count_words(words: String) -> Py<PyAny> {
    let mut hm = HashMap::new();

    for word in words.split_whitespace() {
        let count = hm.entry(word).or_insert(0);

        *count += 1;
    }

    Python::with_gil(|py| hm.to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyword_counter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
