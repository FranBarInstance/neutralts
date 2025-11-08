#![doc = include_str!("../../doc/bif-exit.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:exit; :}
        {:exit; 404 :}
        {:!exit; 202 :} {:* only sets the status code :}
        {:exit; 301 >> /page :}
    */
    pub(crate) fn parse_bif_exit(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        if self.inherit.in_cache {
            self.out = format!("{}{}{}", "{:!cache;", self.raw.to_string(), ":}");
        } else {
            self.out = EMPTY_STRING;
        }

        let has_status_params = self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, false);
        }

        let mut status_code = "200";
        let mut status_param = "";

        if has_status_params {
            if !self.params.is_empty() {
                status_code = self.params.as_str();
            }
            status_param = &self.code;
        } else if !self.code.is_empty() {
            status_code = self.code.as_str();
        }

        self.shared.status_code = status_code.to_string();
        self.shared.status_param = status_param.to_string();

        if let Some(text) = STATUS_CODES.get(status_code) {
            self.shared.status_text = text.to_string();
        } else {
            self.shared.status_text = EMPTY_STRING;
        }

        self.shared.exit = true ^ self.mod_negate;

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_exit_tests.rs"]
mod tests;
