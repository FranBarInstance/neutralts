#![doc = include_str!("../../doc/bif-filled.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*};

impl<'a> Bif<'a> {
    /*
        {:filled; varname >> ... :}

        *** It is only not filled if it has nothing "", if it is not defined
        or is null, the rest "false", "0" etc. is something.
    */
    pub(crate) fn parse_bif_filled(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let mut varname = self.params.as_str();
        let mut schema = &self.shared.schema["data"];

        if varname.starts_with("local::") {
            schema = &self.shared.schema["__indir"][&self.inherit.indir]["data"];
            varname = varname.strip_prefix("local::").unwrap_or(varname);
        }

        if !is_empty_key(schema, varname) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }

            self.out = self.code.to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_filled_tests.rs"]
mod tests;
