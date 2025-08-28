#![doc = include_str!("../../doc/bif-locale.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*, Value};
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:locale; file-path :}
    */
    pub(crate) fn parse_bif_locale(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            if !self.flags.contains("|require|")
                && !self.flags.contains("|inline|")
                && !self.flags.contains("|noparse|")
            {
                return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
            }
        }

        if self.flags.contains("|inline|") {
            // Parse possible bifs included in json
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
            }

            let locale: Value = match serde_json::from_str(&self.code) {
                Ok(value) => value,
                Err(_) => {
                    return Err(self.bif_error(BIF_ERROR_NOT_VALID_JSON));
                }
            };

            let indir = &self.inherit.create_block_schema(self.shared);

            // Merge new locale data in curren locale.
            merge_schema(&mut self.shared.schema["__indir"][indir]["locale"], &locale);

            self.out = EMPTY_STRING;

            return Ok(());
        }

        self.file_path = self.code.clone();

        // For security requires {:allow;
        if self.file_path.contains(BIF_OPEN) {
            if !self.contains_allow(&self.file_path) {
                return Err(self.bif_error(BIF_ERROR_INSECURE_FILE_NAME));
            }
            self.file_path = new_child_parse!(self, &self.code, false);
        }

        if let Some(stripped) = self.file_path.strip_prefix('#') {
            self.file_path = format!("{}{}", self.inherit.current_dir, stripped);
        }

        let path = Path::new(&self.file_path);
        if !Path::new(path).exists() {
            if self.flags.contains("|require|") {
                return Err(self.bif_error(BIF_ERROR_FILE_NOT_FOUND));
            } else {
                return Ok(());
            }
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if self.mod_negate && self.inherit.locale_files.contains(&canonical_path) {
            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        self.inherit.locale_files.push(canonical_path);
        let mut file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());

        if !self.flags.contains("|noparse|") {
            // Parse possible bifs included in json
            if file_raw.contains(BIF_OPEN) {
                file_raw = new_child_parse!(self, &file_raw, false);
            }
        }

        let locale: Value = match serde_json::from_str(&file_raw) {
            Ok(value) => value,
            Err(_) => {
                return Err(self.bif_error(BIF_ERROR_NOT_VALID_JSON));
            }
        };

        let indir = &self.inherit.create_block_schema(self.shared);

        // Merge new locale data in curren locale.
        merge_schema(&mut self.shared.schema["__indir"][indir]["locale"], &locale);

        self.out = UNPRINTABLE.to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_locale_tests.rs"]
mod tests;
