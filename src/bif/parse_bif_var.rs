#![doc = include_str!("../../doc/bif-var.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*};

impl<'a> Bif<'a> {
    /*
        {:;varname:}
        {:;:}
    */
    pub(crate) fn parse_bif_var(&mut self) -> Result<(), BifError> {
        if self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        // Unprintable: {:;:} / {:; :}
        if self.src.is_empty() {
            // "bif.alias" is used and not "bif.name" because in "var" or "unprintable"
            // its name is an empty string.
            self.alias = "unprintable".to_string();

            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        // Var: {:;varname:}
        self.alias = "var".to_string();
        let var_name;

        // For security requires {:allow; in some cases.
        if self.src.contains(BIF_OPEN) {
            if !self.contains_allow(&self.src) {
                self.out = EMPTY_STRING;

                return Err(self.bif_error(BIF_ERROR_INSECURE_VARNAME));
            }

            var_name = new_child_parse!(self, &self.src, self.mod_scope);
        } else {
            var_name = self.src.clone();
        }

        self.out = self.get_data(&var_name).to_string();

        if (self.mod_filter || self.shared.filter_all) && !self.mod_negate {
            if !var_name.starts_with("CONTEXT->") {
                // unescape_chars for prevent double encoding
                self.out = unescape_chars(&self.get_data(&var_name), true).to_string();
                self.out = escape_chars(&self.out, true).to_string();
            }
        } else {
            if self.shared.filter_bifs {
                // Avoid reevaluation in cache
                self.out = self.out.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
                self.out = self.out.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
            }
        }

        if self.mod_negate && !self.shared.filter_bifs && var_name.starts_with("CONTEXT->") {
            self.out = unescape_chars(&self.out, true).to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_var_tests.rs"]
mod tests;
