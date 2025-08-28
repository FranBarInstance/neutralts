#![doc = include_str!("../../doc/bif-contains.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:contains; /haystack/needle/ >> ... :}
    */
    pub(crate) fn parse_bif_contains(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(false);

        if self.params.contains("{:flg;") {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let args = self.extract_args();

        let haystack = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        let needle = args
            .get(2)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        if haystack.contains(&needle) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
            self.out = self.code.to_string();
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_contains_tests.rs"]
mod tests;
