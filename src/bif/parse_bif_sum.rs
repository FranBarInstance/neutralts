#![doc = include_str!("../../doc/bif-sum.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
       {:sum; /a/b/ :}
    */
    pub(crate) fn parse_bif_sum(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.params = self.src.clone();

        if self.params.contains("{:flg;") {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let args = self.extract_args();

        let param1_str = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        let param2_str = args
            .get(2)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        let param1: f64 = param1_str
            .parse()
            .map_err(|_| self.bif_error(BIF_ERROR_INVALID_ARGUMENT_1))?;

        let param2: f64 = param2_str
            .parse()
            .map_err(|_| self.bif_error(BIF_ERROR_INVALID_ARGUMENT_2))?;

        self.out = (param1 + param2).to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_sum_tests.rs"]
mod tests;
