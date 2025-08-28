#![doc = include_str!("../../doc/bif-replace.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:replace; /from/to/ >> ... :}
        /from/to/, ~from~to~, |from|to|, ...
    */
    pub(crate) fn parse_bif_replace(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(false);

        if self.params.contains("{:flg;") {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let args = self.extract_args();

        let from = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        let to = args
            .get(2)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.out = self.code.replace(&from, &to);

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_replace_tests.rs"]
mod tests;
