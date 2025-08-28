#![doc = include_str!("../../doc/bif-data.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*, Value};
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:data; file-path :} {:* local data *}
    */
    pub(crate) fn parse_bif_data(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            if !self.flags.contains("|require|") && !self.flags.contains("|inline|") {
                return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
            }
        }

        if self.flags.contains("|inline|") {
            let data: Value = match serde_json::from_str(&self.code) {
                Ok(value) => value,
                Err(_) => {
                    return Err(self.bif_error(BIF_ERROR_NOT_VALID_JSON));
                }
            };

            let indir = &self.inherit.create_block_schema(self.shared);

            // Merge new locale data in curren local data.
            merge_schema(
                &mut self.shared.schema["__indir"][indir]["data"],
                &data["data"],
            );

            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        self.file_path = self.code.clone();

        // For security requires {:allow;
        if self.file_path.contains(BIF_OPEN) {
            if !self.contains_allow(&self.file_path) {
                return Err(self.bif_error("insecure file name"));
            }
            self.file_path = new_child_parse!(self, &self.code, false);
        }

        if let Some(stripped) = self.file_path.strip_prefix('#') {
            self.file_path = format!("{}{}", self.inherit.current_dir, stripped);
        }

        let path = Path::new(&self.file_path);
        if !Path::new(path).exists() {
            if self.flags.contains("|require|") {
                return Err(self.bif_error("file not found"));
            } else {
                self.out = EMPTY_STRING;

                return Ok(());
            }
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if self.mod_negate && self.inherit.data_files.contains(&canonical_path) {
            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        self.inherit.data_files.push(canonical_path);
        let file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());

        let data: Value = match serde_json::from_str(&file_raw) {
            Ok(value) => value,
            Err(_) => {
                return Err(self.bif_error(BIF_ERROR_NOT_VALID_JSON));
            }
        };

        let indir = &self.inherit.create_block_schema(self.shared);

        // Merge new locale data in curren local data.
        merge_schema(
            &mut self.shared.schema["__indir"][indir]["data"],
            &data["data"],
        );

        self.out = UNPRINTABLE.to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_data_tests.rs"]
mod tests;
