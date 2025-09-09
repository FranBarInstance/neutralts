#![doc = include_str!("../../doc/bif-code.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*};
use std::collections::HashSet;

impl<'a> Bif<'a> {
    /*
        {:code; ...  :}
        {:code; {:flg; safe noparse encode_tags encode_tags_after encode_bifs :} >>  <div>...</div>  :}
    */
    pub(crate) fn parse_bif_code(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            let flags_allowed: HashSet<&str> = [
                "safe",
                "encode_tags",
                "encode_bifs",
                "noparse",
                "encode_tags_after",
            ]
            .into_iter()
            .collect();

            for f in self.flags.split('|').filter(|s| !s.is_empty()) {
                if !flags_allowed.contains(f) {
                    return Err(self.bif_error(&format!("{} flag not allowed", f)));
                }
            }
        }

        if self.flags.contains("|safe|") {
            self.code = escape_chars(&unescape_chars(&self.code, false), false).to_string();
            self.code = self.code.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
            self.code = self.code.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
        } else {
            if self.flags.contains("|encode_tags|") {
                self.code = escape_chars(&unescape_chars(&self.code, false), false).to_string();
            }

            if self.flags.contains("|encode_bifs|") {
                self.code = self.code.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
                self.code = self.code.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
            }

            if !self.flags.contains("|noparse|") && self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
        }

        if self.flags.contains("|encode_tags_after|") {
            self.code = escape_chars(&unescape_chars(&self.code, false), false).to_string();
        }

        self.out = self.code.to_string();

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_code_tests.rs"]
mod tests;
