#![doc = include_str!("../../doc/bif-fetch.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:fetch; |url|event|wrapperId|class|id|name| >> code :}
    */
    pub(crate) fn parse_bif_fetch(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        let args = self.extract_args();

        let url = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND))?;

        let event = args.get(2).cloned().unwrap_or("".to_string());
        let wrap = args.get(3).cloned().unwrap_or("".to_string());
        let class = args.get(4).cloned().unwrap_or("".to_string());
        let id = args.get(5).cloned().unwrap_or("".to_string());
        let name = args.get(6).cloned().unwrap_or("".to_string());
        let div;

        match event.as_str() {
            "form" => div = DIV_FETCH_FORM,
            "none" => div = DIV_FETCH_NONE,
            "visible" => div = DIV_FETCH_VISIBLE,
            "click" => div = DIV_FETCH_CLICK,
            "auto" => div = DIV_FETCH_AUTO,
            _ => div = DIV_FETCH_AUTO,
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.out = div
            .replace("{id}", &id)
            .replace("{name}", &name)
            .replace("{wrap}", &wrap)
            .replace("{class}", &class)
            .replace("{body}", &self.code)
            .replace("{endpoint}", &url);

        if !self.shared.disable_js && !self.shared.already_js {
            self.out = format!(
                "{}{}{}{}",
                self.out,
                "{:!cache;{:moveto;</body>>",
                NEUTRAL_JS.to_string(),
                ":}:}"
            );
            self.shared.already_js = true;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_fetch_tests.rs"]
mod tests;
