#![doc = include_str!("../../doc/bif-obj.md")]

use crate::{
    bif::{constants::*, Bif, BifError, PhpExecutor, PythonExecutor},
    constants::*,
    utils::{is_empty_key, resolve_pointer},
    Value,
};
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:obj; ...  :}
        {:obj; {:flags; inline :} --- >>  <div>...</div>  :}
    */
    pub(crate) fn parse_bif_obj(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        let mut added_bif_code = false;
        if !self.src.contains(BIF_CODE) {
            self.src.push_str(BIF_CODE);
            added_bif_code = true;
        }

        self.extract_params_code(false);

        if added_bif_code {
            self.src.truncate(self.src.len() - BIF_CODE.len());
        }

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let obj_raw;
        if self.params.starts_with('{')
            && self.params.ends_with('}')
            && self.params.chars().nth(1) != Some(':')
            && self.params.chars().nth(self.params.len() - 2) != Some(':')
        {
            // json inline
            obj_raw = self.params.clone();
        } else {
            if self.params.contains(BIF_OPEN) {
                self.params = new_child_parse!(self, &self.params, false);
            }

            self.file_path = self.params.clone();

            // For security requires {:allow;
            if self.file_path.contains(BIF_OPEN) {
                if !self.contains_allow(&self.file_path) {
                    return Err(self.bif_error(BIF_ERROR_INSECURE_FILE_NAME));
                }
                self.file_path = new_child_parse!(self, &self.params, false);
            }

            if let Some(stripped) = self.file_path.strip_prefix('#') {
                self.file_path = format!("{}{}", self.inherit.current_dir, stripped);
            }

            let path = Path::new(&self.file_path);
            if !path.exists() {
                return Err(self.bif_error(BIF_ERROR_FILE_NOT_FOUND));
            }

            obj_raw = fs::read_to_string(&self.file_path)
                .map_err(|e| self.bif_error(&format!("Failed to read file: {}", e)))?;
        }

        let mut obj: Value = serde_json::from_str(obj_raw.trim())
            .map_err(|e| self.bif_error(&format!("Failed to parse JSON: {}", e)))?;

        let engine = obj["engine"]
            .as_str()
            .unwrap_or(DEFAULT_OBJ_ENGINE)
            .to_lowercase();
        if engine != "python" && engine != "php" {
            return Err(self.bif_error(BIF_ERROR_ONLY_PYTHON_ENGINE));
        }

        if !self.flags.contains("|inline|") {
            self.parse_obj_values(&mut obj, false);
        }

        let mut file_path_obj = obj["file"].as_str().unwrap_or("").to_string();

        if let Some(stripped) = file_path_obj.strip_prefix('#') {
            file_path_obj = format!("{}{}", self.inherit.current_dir, stripped);
        }

        if !Path::new(&file_path_obj).exists() {
            return Err(self.bif_error(BIF_ERROR_OBJ_FILE_NOT_FOUND));
        }
        file_path_obj = fs::canonicalize(&file_path_obj)
            .map_err(|e| self.bif_error(&format!("Failed to canonicalize obj script: {}", e)))?
            .to_string_lossy()
            .into_owned();

        let schema_data = obj
            .get("schema_data")
            .and_then(|v| v.as_str())
            .map(|schema_data_name| {
                let (schema_root, key_name) = if schema_data_name.starts_with("local::") {
                    (
                        &self.shared.schema["__indir"][&self.inherit.indir]["data"],
                        schema_data_name.strip_prefix("local::").unwrap_or(""),
                    )
                } else {
                    (&self.shared.schema["data"], schema_data_name)
                };

                resolve_pointer(schema_root, key_name)
                    .cloned()
                    .unwrap_or(Value::Null)
            });

        let schema = if obj.get("schema").and_then(|v| v.as_bool()).unwrap_or(false) {
            Some(&self.shared.schema)
        } else {
            None
        };

        let result = if engine == "python" {
            let mut venv_path = obj["venv"].as_str().unwrap_or("").to_string();

            if !venv_path.is_empty() {
                if let Some(stripped) = venv_path.strip_prefix('#') {
                    venv_path = format!("{}{}", self.inherit.current_dir, stripped);
                }

                if !Path::new(&venv_path).exists() {
                    return Err(self.bif_error("venv path does not exist"));
                }
            }

            PythonExecutor::exec_py(
                &file_path_obj,
                &obj["params"],
                obj["callback"].as_str().unwrap_or(DEFAULT_OBJ_CALLBACK),
                schema,
                schema_data.as_ref(),
                if venv_path.is_empty() {
                    None
                } else {
                    Some(venv_path.as_str())
                },
            )
            .map_err(|e| self.bif_error(&e.msg))?
        } else {
            let mut fpm_endpoint = obj["fpm"].as_str().unwrap_or(DEFAULT_OBJ_FPM).to_string();
            if let Some(stripped) = fpm_endpoint.strip_prefix('#') {
                fpm_endpoint = format!("{}{}", self.inherit.current_dir, stripped);
            }

            PhpExecutor::exec_php(
                &file_path_obj,
                &obj["params"],
                obj["callback"].as_str().unwrap_or(DEFAULT_OBJ_CALLBACK),
                schema,
                schema_data.as_ref(),
                &fpm_endpoint,
            )
            .map_err(|e| self.bif_error(&e.msg))?
        };

        let mut code = String::new();
        if !is_empty_key(&result, "data") {
            let data = serde_json::to_string(&result).unwrap();
            code = String::from("{:data;{:flg; inline :}>>") + &data + ":}";
        }
        if !is_empty_key(&obj, "template") {
            let template = obj["template"].as_str().unwrap();
            code = code + "{:include;" + template + ":}";
        }
        self.code = code + &self.code.clone();

        if self.code.contains(BIF_OPEN) {
            self.out = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.out = self.code.clone();
        }

        Ok(())
    }

    fn parse_obj_values(&mut self, value: &mut Value, is_recursive_call: bool) {
        if let Value::Object(map) = value {
            for (key, val) in map.iter_mut() {
                if key == "file"
                    || key == "template"
                    || key == "venv"
                    || key == "schema_data"
                    || key == "fpm"
                {
                    if let Value::String(s) = val {
                        if s.contains(BIF_OPEN) {
                            *val = Value::String(new_child_parse!(self, s, false));
                        }
                    }
                } else if key == "params" || is_recursive_call {
                    // Only "params" needs recursion.
                    if let Value::String(s) = val {
                        if s.contains(BIF_OPEN) {
                            *val = Value::String(new_child_parse!(self, s, false));
                        }
                    } else if let Value::Object(_) = val {
                        self.parse_obj_values(val, true);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "parse_bif_obj_tests.rs"]
mod tests;
