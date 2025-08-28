#![doc = include_str!("../../doc/bif-join.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, Value};

impl<'a> Bif<'a> {
    /*
        {:join; /array/separator/ :}
        {:join; /array/separator/bool true for join keys instead values/ :}
        <li>{:join; |array|</li><li>| :}</li>
        {:join; /array/ / :}
    */
    pub(crate) fn parse_bif_join(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.params = self.src.clone();
        let args = self.extract_args();
        let mut array_name = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGS_ARRAY_NOT_FOUND))?;
        let separator = args
            .get(2)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGS_SEPARATOR_NOT_FOUND))?;

        // optional use keys
        let use_keys = args.get(3).cloned().unwrap_or("".to_string());

        let keys: bool = match use_keys.as_str() {
            "" => false,
            "false" => false,
            "0" => false,
            _ => true,
        };

        let data_storage;
        array_name = format!("{}{}", "/", array_name);
        array_name = array_name.replace(BIF_ARRAY, "/");
        if array_name.starts_with("/local::") {
            array_name = array_name.replace("/local::", "/");
            data_storage = &self.shared.schema["__indir"][&self.inherit.indir]["data"];
        } else {
            data_storage = &self.shared.schema["data"];
        }

        let mut joined = String::new();
        if let Some(data_value) = data_storage.pointer(&array_name) {
            match data_value.to_owned() {
                Value::Object(obj) => {
                    if keys {
                        joined = obj
                            .keys()
                            .map(|k| k.to_string()) // Convertir cada clave a String
                            .collect::<Vec<String>>()
                            .join(&separator);
                    } else {
                        joined = obj
                            .values()
                            .map(|v| match v {
                                Value::Object(_) => "".to_string(),
                                Value::Array(_) => "".to_string(),
                                Value::String(s) => s.to_string(),
                                Value::Number(n) => n.to_string(),
                                Value::Bool(b) => b.to_string(),
                                _ => v.to_string(),
                            })
                            .collect::<Vec<String>>()
                            .join(&separator);
                    }
                }
                Value::Array(arr) => {
                    if keys {
                        joined = (0..arr.len())
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>()
                            .join(&separator);
                    } else {
                        joined = arr
                            .iter()
                            .map(|v| v.as_str().unwrap_or(""))
                            .collect::<Vec<&str>>()
                            .join(&separator);
                    }
                }
                _ => {}
            }
        }

        self.out = joined.to_string();
        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_join_tests.rs"]
mod tests;
