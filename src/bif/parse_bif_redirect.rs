#![doc = include_str!("../../doc/bif-redirect.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:redirect; 301 >> /page :}
        {:redirect; js:reload:top >> (none) :}
    */
    pub(crate) fn parse_bif_redirect(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope || self.mod_negate {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        if self.inherit.in_cache {
            self.out = format!("{}{}{}", "{:!cache;", self.raw.to_string(), ":}");
        } else {
            self.out = EMPTY_STRING;
        }

        let status_code;
        let has_status_params = self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.src, false);
        }

        if has_status_params {
            // When parameters are required or optional in BIF
            status_code = match self.params.as_str() {
                "301" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "301"
                }
                "302" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "302"
                }
                "303" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "303"
                }
                "307" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }

                    "307"
                }
                "308" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
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
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
                    }
                    // TODO replace(['%2F','%3A','%3F','%3D','%26'], ['/',':','?','=','&'], url);
                    self.shared.redirect_js =
                        REDIR_JS_REDIRECT_TOP.replace("{}", &self.code).to_string();

                    "200"
                }
                "js:redirect:self" => {
                    if self.code.is_empty() {
                        return Err(BifError {
                            msg: "this redirection requires URL".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
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
                        return Err(BifError {
                            msg: "status code not allowed".to_string(),
                            name: self.alias.clone(),
                            src: self.raw.to_string(),
                        });
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
                _ => {
                    return Err(BifError {
                        msg: "redirect type not allowed".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
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
mod tests {
    use crate::test_helpers::*;
    use crate::constants::*;

    #[test]
    fn test_bif_redirect_301() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; 301 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "301");
        assert_eq!(template.get_status_text(), "Moved Permanently");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "301 Moved Permanently\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_302() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; 302 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "302");
        assert_eq!(template.get_status_text(), "Found");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "302 Found\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_303() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; 303 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "303");
        assert_eq!(template.get_status_text(), "See Other");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "303 See Other\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_307() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; 307 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "307");
        assert_eq!(template.get_status_text(), "Temporary Redirect");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "307 Temporary Redirect\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_308() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; 308 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "308");
        assert_eq!(template.get_status_text(), "Permanent Redirect");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "308 Permanent Redirect\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_js_reload_top() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template
            .set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:top :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "js:reload:top");
        assert_eq!(result, REDIR_JS_RELOAD_TOP);
    }

    #[test]
    fn test_bif_redirect_js_reload_top_param() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; js:reload:top >> some :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "some");
        assert_eq!(result, REDIR_JS_RELOAD_TOP);
    }

    #[test]
    fn test_bif_redirect_js_reload_self() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template
            .set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:self :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "js:reload:self");
        assert_eq!(result, REDIR_JS_RELOAD_SELF);
    }

    #[test]
    fn test_bif_redirect_js_reload_self_param() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(
            "<div>{:;__test-nts:}{:redirect; js:reload:self >> some :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "some");
        assert_eq!(result, REDIR_JS_RELOAD_SELF);
    }

    #[test]
    fn test_bif_redirect_negate_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template
            .set_src_str("<div>{:;__test-nts:}{:!redirect; js:reload:top :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_redirect_fails_no_params_1() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:!redirect; :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_redirect_fails_no_params_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:!redirect; 301 >> :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }
}
