#![doc = include_str!("../../doc/bif-moveto.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, json};
use md5::{Digest, Md5};

impl<'a> Bif<'a> {
    /*
        {:moveto; <tag >> ... :}
        {:moveto; </tag >> ... :}
    */
    pub(crate) fn parse_bif_moveto(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        if self.inherit.in_cache {
            self.out = format!("{}{}{}", "{:!cache;", self.raw.to_string(), ":}");
        } else {
            self.out = EMPTY_STRING;
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.moveto(&self.params.clone(), &self.code.clone());

        Ok(())
    }

    pub(crate) fn moveto(&mut self, to: &str, code: &str) {
        let mut moveto = json!({});
        let mut hasher = Md5::new();

        // the same code moves only once
        hasher.update(code.replace("\n", "").replace(" ", ""));
        let code_hash = hasher.finalize();
        let code_hash = format!("{:x}", code_hash);

        moveto[to] = json!(code);
        self.shared.schema["__moveto"][&code_hash] = moveto;
    }
}

#[cfg(test)]
#[path = "parse_bif_moveto_tests.rs"]
mod tests;
