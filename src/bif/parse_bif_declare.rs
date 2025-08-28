#![doc = include_str!("../../doc/bif-declare.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, json};

impl<'a> Bif<'a> {
    /*
        {:declare; name >> words list :}
    */
    pub(crate) fn parse_bif_declare(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.inherit.current_file.contains(SNIPPETS_FILES) {
            self.inherit.create_block_schema(self.shared);
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
                self.code = self.code.replace(UNPRINTABLE, "");
            }
            self.shared.schema["__indir"][&self.inherit.indir]["declare"][&self.params] =
                json!(&self.code);

            self.out = EMPTY_STRING;
        } else {
            return Err(self.bif_error(BIF_ERROR_DECLARE_SET_HERE));
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_declare_tests.rs"]
mod tests;
