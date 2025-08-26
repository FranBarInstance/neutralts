#![doc = include_str!("../../doc/bif-obj.md")]

use crate::{
    bif::{Bif, BifError, PythonExecutor},
    constants::*,
    utils::is_empty_key,
    Value,
};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:obj; ...  :}
        {:code; {:flags; inline :} --- >>  <div>...</div>  :}
    */
    pub(crate) fn parse_bif_obj(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let mut added_bif_code = false;
        if !self.src.contains(BIF_CODE) {
            self.src.push_str(BIF_CODE);
            added_bif_code = true;
        }

        self.extract_params_code(true);

        if added_bif_code {
            self.src.truncate(self.src.len() - BIF_CODE.len());
        }

        if !self.flags.is_empty() {
            let flags_allowed: HashSet<&str> = ["inline"].into_iter().collect();

            for f in self.flags.split('|').filter(|s| !s.is_empty()) {
                if !flags_allowed.contains(f) {
                    return Err(BifError {
                        msg: format!("{} flag not allowed", f),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    });
                }
            }
        }

        let obj_raw;
        if self.flags.contains("|inline|") {
            obj_raw = self.params.clone();
        } else {
            self.file_path = self.params.clone();

            // For security requires {:allow;
            if self.file_path.contains(BIF_OPEN) {
                if !self.contains_allow(&self.file_path) {
                    return Err(BifError {
                        msg: "insecure file name".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    });
                }
                self.file_path = new_child_parse!(self, &self.params, false);
            }

            if let Some(stripped) = self.file_path.strip_prefix('#') {
                self.file_path = format!("{}{}", self.inherit.current_dir, stripped);
            }

            let path = Path::new(&self.file_path);
            if !path.exists() {
                return Err(BifError {
                    msg: "file not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }

            obj_raw = fs::read_to_string(&self.file_path).map_err(|e| BifError {
                msg: format!("Failed to read file: {}", e),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            })?;
        }

        let mut obj: Value = serde_json::from_str(obj_raw.trim()).map_err(|e| BifError {
            msg: format!("Failed to parse JSON: {}", e),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        let engine = obj["engine"].as_str().unwrap_or(DEFAULT_OBJ_ENGINE);
        if engine.to_lowercase() != "python" {
            // currently only Python is supported
            return Err(BifError {
                msg: "only Python engine is supported".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        if !self.flags.contains("|inline|") {
            self.parse_obj_values(&mut obj, false);
        }

        let mut file_path_obj = obj["file"].as_str().unwrap_or("").to_string();

        if let Some(stripped) = file_path_obj.strip_prefix('#') {
            file_path_obj = format!("{}{}", self.inherit.current_dir, stripped);
        }

        if !Path::new(&file_path_obj).exists() {
            return Err(BifError {
                msg: "obj file not found".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let params = &obj["params"];
        let callback_name = obj["callback"].as_str().unwrap_or(DEFAULT_OBJ_CALLBACK);
        let result = PythonExecutor::exec_py(
            &file_path_obj,
            params,
            callback_name,
            Some(&self.shared.schema)
        )?;

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
                if key == "file" || key == "template" {
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
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_obj() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; inline :} { \"file\": \"tests/script.py\" } >> {:;local::py_hello:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; inline :} { \"file\": \"tests/script.py\" } >>  :}{:;local::py_hello:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+obj; {:flg; inline :} { \"file\": \"tests/script.py\" } >>  :}{:;local::py_hello:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_objfile() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; tests/obj.json :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_invalid_file() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; inline :} { \"file\": \"nonexistent.py\" } >> {:;local::py_hello:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; unknown :} { \"file\": \"tests/script.py\" } >> {:;local::py_hello:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_insecure_filename() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:;dangerous:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_invalid_json() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; inline :} {malformed json} >> :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_unsupported_engine() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; inline :} {\"engine\":\"ruby\",\"file\":\"tests/script.py\"} >> :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_missing_obj_file() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; nonexistent.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_template_integration() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; tests/obj.json >> |Inline:{:;local::py_hello:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!|Inline:Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_with_params() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; inline :} { \"file\": \"tests/script.py\", \"params\": {\"param1\":\"{:;__test-nts:}\"} } >> {:;local::param1:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }
}
