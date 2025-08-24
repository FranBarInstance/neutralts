#![doc = include_str!("../../doc/bif-each.md")]

use crate::{bif::Bif, bif::BifError, constants::*, json, Value};

impl<'a> Bif<'a> {
    /*
        {:each; array-name name-for-key name-for-value  >>
            {:;name-for-key:}={:;name-for-value:}
        :}
    */
    pub(crate) fn parse_bif_each(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        let mut parts = self.params.split_whitespace();

        let array_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    msg: "arguments not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let key_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    msg: "arguments 'key' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let val_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    msg: "arguments 'value' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let tmp: String = format!("{}{}", "/", array_name);
        let mut array = tmp.replace(BIF_ARRAY, "/");
        let restore_key = self.get_data(&key_name);
        let restore_val = self.get_data(&val_name);

        let data_storage;
        if array.starts_with("/local::") {
            array = array.replace("/local::", "/");
            data_storage = &self.shared.schema["__indir"][&self.inherit.indir]["data"];
        } else {
            data_storage = &self.shared.schema["data"];
        }

        if let Some(data_value) = data_storage.pointer(&array) {
            match data_value.to_owned() {
                Value::Object(obj) => {
                    for (key, val) in obj.iter() {
                        self.parse_bif_each_iter(&key_name, &val_name, key, val);
                    }
                }
                Value::Array(arr) => {
                    for (key, val) in arr.iter().enumerate() {
                        self.parse_bif_each_iter(&key_name, &val_name, &key.to_string(), val);
                    }
                }
                _ => {}
            }
        }

        self.set_data(&key_name, &restore_key);
        self.set_data(&val_name, &restore_val);

        Ok(())
    }

    fn parse_bif_each_iter(
        &mut self,
        key_name: &str,
        val_name: &str,
        key: &String,
        val: &Value,
    ) {
        self.shared.schema["data"][key_name] = json!(key);
        self.shared.schema["data"][val_name] = json!(val);
        self.out += &new_child_parse!(self, &self.code, self.mod_scope);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_each() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:each; __test-obj-nts->level1-obj->level2-obj->level3-arr key value >> {:;key:}={:;value:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0=one1=two2=three</div>");
    }

    #[test]
    fn test_bif_each_iterate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:code; {:param; array-name >> __test-obj-nts :} {:snippet; iterate-array :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
        result,
        "<div>\n        level1=Ok\n        level1-obj:\n                level1=Ok\n                level2-obj:\n                        level2=Ok\n                        level3-arr:\n                                0=one\n                                1=two\n                                2=three</div>"
    );
    }

    #[test]
    fn test_bif_each_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:each; __test-arr-nts key val >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_each_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+each; __test-arr-nts key val >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

}
