#![doc = include_str!("../../doc/bif-allow.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*};
use std::collections::HashSet;

impl<'a> Bif<'a> {
    /*
        {:allow; {:flg; partial casein replace :} name >> ... :}
    */
    pub(crate) fn parse_bif_allow(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);
        let mut found = String::new();
        let words_string = get_from_key(
            &self.shared.schema["__indir"][&self.inherit.indir]["declare"],
            &self.params,
        );

        if words_string.is_empty() {
            return Err(self.bif_error(&(self.params.clone() + BIF_ERROR_DECLARED_IS_EMPTY)));
        }

        if !self.flags.is_empty() {
            let flags_allowed: HashSet<&str> =
                ["partial", "replace", "casein"].into_iter().collect();

            for f in self.flags.split('|').filter(|s| !s.is_empty()) {
                if !flags_allowed.contains(f) {
                    return Err(self.bif_error(&format!("{} flag not allowed", f)));
                }
            }
        }

        let mut words_list: Vec<&str> = words_string.split_whitespace().collect();
        self.code = new_child_parse!(self, &self.code, self.mod_scope);

        for word in &mut words_list {
            let lower_haystack;
            let mut haystack = &self.code;
            let mut pattern = word.to_string().clone();

            if self.flags.contains("|partial|") || self.flags.contains("|replace|") {
                pattern = format!("{}{}{}", "*", pattern, "*");
            }

            if self.flags.contains("|casein|") {
                pattern = pattern.to_lowercase();
                lower_haystack = self.code.clone().to_lowercase();
                haystack = &lower_haystack;
            }

            if wildcard_match(haystack, &pattern) {
                found = word.to_string();
                break;
            }
        }

        if !found.is_empty() ^ self.mod_negate {
            if self.flags.contains("|replace|") {
                found = found.replace("~", "");
                found = found.replace("*", "");
                found = found.replace("?", "");
                found = found.replace(".", "");
                self.out = found.to_string();
            } else {
                self.out = self.code.to_string();
            }
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_allow_tests.rs"]
mod tests;
