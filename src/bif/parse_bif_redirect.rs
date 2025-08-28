#![doc = include_str!("../../doc/bif-redirect.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:redirect; 301 >> /page :}
        {:redirect; js:reload:top >> (none) :}
    */
    pub(crate) fn parse_bif_redirect(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        if self.inherit.in_cache {
            self.out = format!("{}{}{}", "{:!cache;", self.raw.to_string(), ":}");
        } else {
            self.out = EMPTY_STRING;
        }

        let status_code;
        let has_status_params = self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.src, false);
        }

        if has_status_params {
            // When parameters are required or optional in BIF
            status_code = match self.params.as_str() {
                "301" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }

                    "301"
                }
                "302" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }

                    "302"
                }
                "303" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }

                    "303"
                }
                "307" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }

                    "307"
                }
                "308" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }

                    "308"
                }
                "js:reload:top" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_TOP.to_string();

                    "200"
                }
                "js:reload:self" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_SELF.to_string();

                    "200"
                }
                "js:redirect:top" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }
                    // TODO replace(['%2F','%3A','%3F','%3D','%26'], ['/',':','?','=','&'], url);
                    self.shared.redirect_js =
                        REDIR_JS_REDIRECT_TOP.replace("{}", &self.code).to_string();

                    "200"
                }
                "js:redirect:self" => {
                    if self.code.is_empty() {
                        return Err(self.bif_error(BIF_ERROR_REDIRECT_REQUIRES_URL));
                    }
                    // TODO replace(['%2F','%3A','%3F','%3D','%26'], ['/',':','?','=','&'], url);
                    self.shared.redirect_js =
                        REDIR_JS_REDIRECT_SELF.replace("{}", &self.code).to_string();

                    "200"
                }
                _ => {
                    // Parameters are optional in js:reload:self and js:reload:top
                    if !self.code.contains("js:reload:self") || !self.code.contains("js:reload:top")
                    {
                        return Err(self.bif_error(BIF_ERROR_STATUS_CODE_NOT_ALLOWED));
                    } else {
                        "200"
                    }
                }
            };
        } else {
            // When parameters are not needed in BIF
            status_code = match self.code.as_str() {
                "js:reload:top" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_TOP.to_string();

                    "200"
                }
                "js:reload:self" => {
                    self.shared.redirect_js = REDIR_JS_RELOAD_SELF.to_string();

                    "200"
                }
                _ => return Err(self.bif_error(BIF_ERROR_REDIRECT_TYPE_NOT_ALLOWED)),
            };
        }

        self.shared.status_param = self.code.to_string();
        self.shared.status_code = status_code.to_string();

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
#[path = "parse_bif_redirect_tests.rs"]
mod tests;
