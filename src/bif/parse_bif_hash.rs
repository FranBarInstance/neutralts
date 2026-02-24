#![doc = include_str!("../../doc/bif-hash.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};
use md5::{Digest, Md5};
use rand::RngExt;

impl<'a> Bif<'a> {
    /*
        {:hash;  :}
        {:hash; text :}
    */
    pub(crate) fn parse_bif_hash(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.code = self.src.trim().to_string();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.code.trim().to_string();
        }

        if self.code.is_empty() {
            let mut hasher = Md5::new();
            let mut rng = rand::rng();
            let rand = rng.random_range(100000000..=999999999).to_string();
            hasher.update(&rand);
            self.out = format!("{:x}", hasher.finalize())
        } else {
            let mut hasher = Md5::new();
            hasher.update(&self.code);
            self.out = format!("{:x}", hasher.finalize());
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_hash_tests.rs"]
mod tests;
