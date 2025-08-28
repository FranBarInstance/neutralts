#![doc = include_str!("../../doc/bif-each.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, json, Value};

impl<'a> Bif<'a> {
    /*
        {:each; array-name name-for-key name-for-value  >>
            {:;name-for-key:}={:;name-for-value:}
        :}
    */
    pub(crate) fn parse_bif_each(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let mut parts = self.params.split_whitespace();

        let array_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND));
            }
        };

        let key_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(self.bif_error(BIF_ERROR_ARGS_KEY_NOT_FOUND));
            }
        };

        let val_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(self.bif_error(BIF_ERROR_ARGS_VALUE_NOT_FOUND));
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

    fn parse_bif_each_iter(&mut self, key_name: &str, val_name: &str, key: &String, val: &Value) {
        self.shared.schema["data"][key_name] = json!(key);
        self.shared.schema["data"][val_name] = json!(val);
        self.out += &new_child_parse!(self, &self.code, self.mod_scope);
    }
}

#[cfg(test)]
#[path = "parse_bif_each_tests.rs"]
mod tests;
