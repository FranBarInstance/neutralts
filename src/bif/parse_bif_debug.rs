#![doc = include_str!("../../doc/bif-debug.md")]

use crate::{
    bif::constants::*,
    bif::Bif,
    bif::BifError,
    constants::*,
    utils::*,
};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, Duration};

impl<'a> Bif<'a> {
    /*
        {:debug; data->key :}
    */
    pub(crate) fn parse_bif_debug(&mut self) -> Result<(), BifError> {
        if self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        let debug_enable = self.debug_enable();

        if self.mod_negate {
            if debug_enable {
                self.out = UNPRINTABLE.to_string();
            } else {
                self.out = EMPTY_STRING;
            }

            return Ok(());
        }

        if !debug_enable {
            self.out = EMPTY_STRING;
            if self.mod_negate {
                return Ok(());
            } else {
                return Err(self.bif_error("Debug is disabled. Remember to remove the bif debug in production."));
            }
        }

        self.extract_params_code(true);

        let mut schema = &self.shared.schema;
        let mut key_name = self.code.clone();
        if key_name.starts_with("local::") {
            key_name = key_name.strip_prefix("local::").unwrap_or("").to_string();
            schema = &self.shared.schema["__indir"][&self.inherit.indir];
        }

        let k = if self.code.is_empty() {
            self.out = VERSION.to_string();
            return Ok(());
        } else if key_name == "full-schema" {
            "".to_string()
        } else {
            format!("/{}", key_name).replace(BIF_ARRAY, "/")
        };

        self.out = match schema.pointer(&k) {
            Some(value) => match serde_json::to_string_pretty(value) {
                Ok(json_str) => json_str,
                Err(e) => format!("Error formatting JSON: {}", e),
            },
            None => format!("Undefined: '{}'", self.code),
        };

        if self.mod_filter {
            // unescape_chars for prevent double encoding
            let tmp = unescape_chars(&self.out, true);
            self.out = escape_chars(&tmp, true);
        }

        Ok(())
    }

    /// check if debug is enabled
    pub(crate) fn debug_enable(&self) -> bool {
        if self.shared.debug_file.is_empty() {
            return false;
        }

        let path = Path::new(&self.shared.debug_file);

        if !path.exists() || !path.is_file() {
            return false;
        }

        let metadata = match fs::metadata(path) {
            Ok(md) => md,
            Err(_) => return false,
        };

        let modified_time = match metadata.modified() {
            Ok(time) => time,
            Err(_) => return false,
        };

        let now = SystemTime::now();

        match now.duration_since(modified_time) {
            Ok(duration) => duration < Duration::from_secs(self.shared.debug_expire),
            Err(_) => false,
        }
    }
}


#[cfg(test)]
#[path = "parse_bif_debug_tests.rs"]
mod tests;
