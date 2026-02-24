#![doc = include_str!("../../doc/bif-each.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, json, utils::resolve_pointer, utils::extract_blocks, Value};

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

        let restore_key = self.get_data(&key_name);
        let restore_val = self.get_data(&val_name);

        let data_storage = if array_name.starts_with("local::") {
            &self.shared.schema["__indir"][&self.inherit.indir]["data"]
        } else {
            &self.shared.schema["data"]
        };

        let array_clean = array_name.strip_prefix("local::").unwrap_or(&array_name);

        let collection = if let Some(data_value) = resolve_pointer(data_storage, array_clean) {
            data_value.clone()
        } else {
            Value::Null
        };

        let blocks = match extract_blocks(&self.code) {
            Ok(b) => b,
            Err(p) => return Err(self.bif_error(&format!("Unmatched block at position {}", p))),
        };

        match collection {
            Value::Object(obj) => {
                for (key, val) in obj.iter() {
                    self.parse_bif_each_iter(&key_name, &val_name, key, val, &blocks);
                }
            }
            Value::Array(arr) => {
                for (idx, val) in arr.iter().enumerate() {
                    self.parse_bif_each_iter(&key_name, &val_name, &idx.to_string(), val, &blocks);
                }
            }
            _ => {}
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
        blocks: &Vec<(usize, usize)>,
    ) {
        self.shared.schema["data"][key_name] = json!(key);
        self.shared.schema["data"][val_name] = json!(val);

        let mut child_inherit = self.inherit.clone();
        child_inherit.alias = self.alias.clone();
        if !self.file_path.is_empty() {
            child_inherit.current_file = self.file_path.clone();
        }
        if !self.dir.is_empty() {
            child_inherit.current_dir = self.dir.clone();
        }

        if self.mod_scope {
            self.inherit.create_block_schema(self.shared);
        }

        let mut block_parser =
            crate::block_parser::BlockParser::new(self.shared, &child_inherit);
        let code = block_parser.parse_with_blocks(&self.code, blocks, self.only);

        if self.mod_scope {
            block_parser.update_indir(&self.inherit.indir);
        }

        self.out += &code;
    }
}

#[cfg(test)]
#[path = "parse_bif_each_tests.rs"]
mod tests;
