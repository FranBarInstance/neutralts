#![doc = include_str!("../../doc/bif-coalesce.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
       {:coalesce;
           {:code;  :}
           {:code; this is output :}
           {:code; ... :}
       :}
    */
    pub(crate) fn parse_bif_coalesce(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        // This var so as not to overwrite the original: inherit.last_bif_out
        self.inherit.last_coalesce_out = false;
        self.out = new_child_parse!(self, &self.src, self.mod_scope);

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_coalesce_tests.rs"]
mod tests;
