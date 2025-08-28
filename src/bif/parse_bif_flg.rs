#![doc = include_str!("../../doc/bif-flg.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:flg; flag-name1 flag-name2 ... :}
        {:code; {:flg; safe :} >>  <div>...</div> :}
    */
    pub(crate) fn parse_bif_flg(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_upline || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, false);
        }

        let flags = format!(" {} ", self.code);
        self.shared.flags = flags.replace(" ", "|");
        self.out = EMPTY_STRING;

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_flg_tests.rs"]
mod tests;
