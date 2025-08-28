#![doc = include_str!("../../doc/bif-same.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:same; /a/b/ >> ... :}
    */
    pub(crate) fn parse_bif_same(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(false);

        if self.params.contains("{:flg;") {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let args = self.extract_args();

        let param1 = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        let param2 = args
            .get(2)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        if (param1 == param2) ^ self.mod_negate {
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
#[path = "parse_bif_same_tests.rs"]
mod tests;
