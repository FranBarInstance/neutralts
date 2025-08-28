#![doc = include_str!("../../doc/bif-lang.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
       {:lang; ... :}
    */
    pub(crate) fn parse_bif_lang(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.out = self.shared.lang.to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_lang_tests.rs"]
mod tests;
