#![doc = include_str!("../../doc/bif-exit.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:exit; :}
        {:exit; 404 :}
        {:!exit; 202 :} {:* only sets the status code :}
        {:exit; 301 >> /page :}
    */
    pub(crate) fn parse_bif_exit(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
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

        let has_status_params = self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.src, false);
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
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_exit() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>nts");
    }

    #[test]
    fn test_bif_exit_custom_status() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 1600 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "1600");
        assert_eq!(template.get_status_text(), "");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>nts");
    }

    #[test]
    fn test_bif_exit_custom_status_param() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 1600 >> some :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "1600");
        assert_eq!(template.get_status_text(), "");
        assert_eq!(template.get_status_param(), "some");
        assert_eq!(result, "<div>nts");
    }

    #[test]
    fn test_bif_exit_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:!exit; :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_exit_202_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:!exit; 202 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "202");
        assert_eq!(template.get_status_text(), "Accepted");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_exit_206() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 206 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "206");
        assert_eq!(template.get_status_text(), "Partial Content");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>nts");
    }

    #[test]
    fn test_bif_exit_301() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 301 >> /home :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "301");
        assert_eq!(template.get_status_text(), "Moved Permanently");
        assert_eq!(template.get_status_param(), "/home");
        assert_eq!(result, "301 Moved Permanently\n/home");
    }

    #[test]
    fn test_bif_exit_302() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 302 >> /home :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "302");
        assert_eq!(template.get_status_text(), "Found");
        assert_eq!(template.get_status_param(), "/home");
        assert_eq!(result, "302 Found\n/home");
    }

    #[test]
    fn test_bif_exit_303() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 303 >> /home :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "303");
        assert_eq!(template.get_status_text(), "See Other");
        assert_eq!(template.get_status_param(), "/home");
        assert_eq!(result, "303 See Other\n/home");
    }

    #[test]
    fn test_bif_exit_307() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 307 >> /home :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "307");
        assert_eq!(template.get_status_text(), "Temporary Redirect");
        assert_eq!(template.get_status_param(), "/home");
        assert_eq!(result, "307 Temporary Redirect\n/home");
    }

    #[test]
    fn test_bif_exit_308() {
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
            "<div>{:;__test-nts:}{:exit; 308 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "308");
        assert_eq!(template.get_status_text(), "Permanent Redirect");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "308 Permanent Redirect\nhttps://example.com/");
    }

    #[test]
    fn test_bif_exit_401() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 401 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "401");
        assert_eq!(template.get_status_text(), "Unauthorized");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "401 Unauthorized");
    }

    #[test]
    fn test_bif_exit_403() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 403 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "403");
        assert_eq!(template.get_status_text(), "Forbidden");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "403 Forbidden");
    }

    #[test]
    fn test_bif_exit_404() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 404 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "404");
        assert_eq!(template.get_status_text(), "Not Found");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "404 Not Found");
    }

    #[test]
    fn test_bif_exit_500() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 500 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "500");
        assert_eq!(template.get_status_text(), "Internal Server Error");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "500 Internal Server Error");
    }

    #[test]
    fn test_bif_exit_503() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:exit; 503 :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "503");
        assert_eq!(template.get_status_text(), "Service Unavailable");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "503 Service Unavailable");
    }
}
