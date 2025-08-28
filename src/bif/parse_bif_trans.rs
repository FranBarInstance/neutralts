#![doc = include_str!("../../doc/bif-trans.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
       {:trans; ... :}
    */
    pub(crate) fn parse_bif_trans(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        // For performance, we avoid calling BlockParser::new if it is not necessary
        if self.src.contains(BIF_OPEN) {
            self.src = new_child_parse!(self, &self.src, self.mod_scope);
        }

        let trans = self.get_trans(&self.src);

        // By default the input text
        if trans.is_empty() {
            if self.mod_negate {
                self.out = EMPTY_STRING;
            } else {
                self.out = self.src.clone();
            }
        } else {
            self.out = trans;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_trans_tests.rs"]
mod tests;
