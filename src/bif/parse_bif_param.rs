#![doc = include_str!("../../doc/bif-param.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, json, utils::*};

impl<'a> Bif<'a> {
    /*
        Play param: {:param; param-name :}
        Set param:  {:param; param-name >> content to set :}
    */
    pub(crate) fn parse_bif_param(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        let is_set = self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if is_set {
            if self.inherit.alias == "code" {
                if self.code.contains(BIF_OPEN) {
                    self.code = new_child_parse!(self, &self.code, self.mod_scope);
                }

                self.inherit.create_block_schema(self.shared);
                self.shared.schema["__indir"][&self.inherit.indir]["params"][&self.params] =
                    json!(&self.code);
                self.out = EMPTY_STRING;

                Ok(())
            } else {
                Err(self.bif_error(BIF_ERROR_PARAM_SET_HERE))
            }
        } else {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }

            self.code = get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["params"],
                &self.code,
            );
            self.out = self.code.to_string();

            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "parse_bif_param_tests.rs"]
mod tests;
