#![doc = include_str!("../../doc/bif-else.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
       {:else; ... :}
       {:code; :}{:else; this is output :}
       {:code; not empty :}{:!else; this is output :}
    */
    pub(crate) fn parse_bif_else(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.inherit.last_bif_out ^ self.mod_negate {
            self.out = EMPTY_STRING;

            return Ok(());
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.out = self.code.to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_else_tests.rs"]
mod tests;
