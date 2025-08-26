// pyo3 = { version = "0.25.1", features = ["auto-initialize"] }

use crate::{bif::BifError, Value};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use pyo3::PyObject;
use std::path::Path;

pub struct PythonExecutor;

impl PythonExecutor {
    pub(crate) fn exec_py(
        file: &str,
        params_value: &Value,
        callback_name: &str,
        schema: Option<&Value>,
    ) -> Result<Value, BifError> {
        Python::with_gil(|py| -> PyResult<Value> {
            let params = Self::prepare_python_params(py, params_value)?;
            Self::setup_python_path(py, file)?;
            Self::execute_python_callback(py, file, callback_name, params, schema)
        })
        .map_err(|e| BifError {
            msg: format!(
                "Error executing callback function '{}': {}",
                callback_name, e
            ),
            name: "python_callback".to_string(),
            src: file.to_string(),
        })
    }

    fn prepare_python_params<'py>(py: Python<'py>, params_value: &Value) -> PyResult<PyObject> {
        let params_json = serde_json::to_string(params_value).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize params: {}", e))
        })?;
        let json_mod = PyModule::import(py, "json")?;
        let loads = json_mod.getattr("loads")?;
        let py_obj = loads.call1((params_json,))?;
        let py_object: PyObject = py_obj.extract()?;
        Ok(py_object)
    }

    fn setup_python_path(py: Python, file: &str) -> PyResult<()> {
        let dir_path = Path::new(file).parent().unwrap_or_else(|| Path::new("."));
        let sys = PyModule::import(py, "sys")?;
        let path_attr = sys.getattr("path")?;
        let path = path_attr.downcast::<PyList>()?;
        if let Some(dir_str) = dir_path.to_str() {
            path.append(dir_str)?;
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid directory path encoding",
            ));
        }
        Ok(())
    }

    fn execute_python_callback<'py>(
        py: Python<'py>,
        file: &str,
        callback_name: &str,
        params: PyObject,
        schema: Option<&Value>,
    ) -> PyResult<Value> {
        let module_name = Self::extract_module_name(file)?;
        let module = PyModule::import(py, &module_name)?;

        if let Some(schema_value) = schema {
            let schema_py = Self::prepare_python_params(py, schema_value)?;
            module.setattr("__NEUTRAL_SCHEMA__", schema_py)?;
        }

        let callback_func = module.getattr(callback_name).map_err(|_| {
            pyo3::exceptions::PyAttributeError::new_err(format!(
                "Module does not have function '{}'",
                callback_name
            ))
        })?;
        let result_any = callback_func.call1((params,))?;
        let result_obj: PyObject = result_any.extract()?;
        Self::convert_python_result_to_json(py, result_obj)
    }

    fn extract_module_name(file: &str) -> PyResult<String> {
        Path::new(file)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                pyo3::exceptions::PyValueError::new_err(
                    "Could not extract module name from file path",
                )
            })
    }

    fn convert_python_result_to_json<'py>(py: Python<'py>, result: PyObject) -> PyResult<Value> {
        let json_module = PyModule::import(py, "json")?;
        let json_dumps = json_module.getattr("dumps")?;
        let json_string: String = json_dumps.call1((result,))?.extract()?;
        serde_json::from_str(&json_string).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Error parsing JSON: {}", e))
        })
    }
}
