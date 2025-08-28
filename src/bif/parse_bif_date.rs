#![doc = include_str!("../../doc/bif-date.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};
use chrono::Utc;

impl<'a> Bif<'a> {
    /*
        {:date;  :} timestamp
        {:date; %Y-%m-%d %H:%M:%S  :} UTC
    */
    pub(crate) fn parse_bif_date(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let now = Utc::now();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.src.trim().to_string();
        }

        if self.code.is_empty() {
            self.out = now.timestamp().to_string();
        } else {
            self.out = now.format(&self.src).to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_date_tests.rs"]
mod tests;
