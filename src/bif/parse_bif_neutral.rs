#![doc = include_str!("../../doc/bif-neutral.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError};

/*
    {:neutral; ... :}
*/

impl<'a> Bif<'a> {
    pub(crate) fn parse_bif_neutral(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.out = self.raw.to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_neutral_tests.rs"]
mod tests;
