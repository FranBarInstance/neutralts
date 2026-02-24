#![doc = include_str!("../../doc/bif-eval.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:eval; code >> ... {:;__eval__:} ... :} {:* embbedding *:}
    */
    pub(crate) fn parse_bif_eval(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(false);

        if self.params.contains("{:flg;") {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.params.contains(BIF_OPEN) {
            self.params = new_child_parse!(self, &self.params, self.mod_scope);
        }

        if (self.params != EMPTY_STRING) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                let restore_eval = self.get_data("__eval__");
                let val = self.params.clone();
                self.set_data("__eval__", &val);
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
                self.set_data("__eval__", &restore_eval);
            }
            self.out = self.code.to_string();
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_eval_tests.rs"]
mod tests;
