#![doc = include_str!("../../doc/bif-fetch.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:fetch; |url|event|wrapperId|class|id|name| >> code :}
    */
    pub(crate) fn parse_bif_fetch(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(BifError {
                msg: "flags not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let args = self.extract_args();

        let url = args.get(1).cloned().ok_or_else(|| BifError {
            msg: "argument 'url' not found".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

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
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_fetch() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; '/url' >> loading... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"\" class=\"neutral-fetch-auto \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; '/{:;__test-nts:}' >> {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"\" class=\"neutral-fetch-auto \" data-url=\"/nts\" data-wrap=\"\">\n    nts\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_form() {
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
            "<div>{:fetch; |/url|form| >> <input type=\"text\" name=\"name\"> :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><form id=\"\" name=\"\" class=\"neutral-fetch-form \" method=\"POST\" action=\"/url\" data-wrap=\"\">\n    <input type=\"text\" name=\"name\">\n</form>\n</div>");
    }

    #[test]
    fn test_bif_fetch_form_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/{:;__test-nts:}|form|wrap-{:;__test-nts:}|class-{:;__test-nts:}|id-{:;__test-nts:}|name-{:;__test-nts:}| >> <input type=\"text\" name=\"name\"> :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><form id=\"id-nts\" name=\"name-nts\" class=\"neutral-fetch-form class-nts\" method=\"POST\" action=\"/nts\" data-wrap=\"wrap-nts\">\n    <input type=\"text\" name=\"name\">\n</form>\n</div>");
    }

    #[test]
    fn test_bif_fetch_none() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/url|none| >> loading... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"\" class=\"neutral-fetch-none \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_none_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/{:;__test-nts:}|none|wrap-{:;__test-nts:}|class-{:;__test-nts:}|id-{:;__test-nts:}| >> {:;__test-nts:}... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"id-nts\" class=\"neutral-fetch-none class-nts\" data-url=\"/nts\" data-wrap=\"wrap-nts\">\n    nts...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_visible() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/url|visible| >> loading... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"\" class=\"neutral-fetch-visible \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_visible_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/{:;__test-nts:}|visible|wrap-{:;__test-nts:}|class-{:;__test-nts:}|id-{:;__test-nts:}| >> {:;__test-nts:}... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"id-nts\" class=\"neutral-fetch-visible class-nts\" data-url=\"/nts\" data-wrap=\"wrap-nts\">\n    nts...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_click() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/url|click| >> loading... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"\" class=\"neutral-fetch-click \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_click_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/{:;__test-nts:}|click|wrap-{:;__test-nts:}|class-{:;__test-nts:}|id-{:;__test-nts:}| >> {:;__test-nts:}... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"id-nts\" class=\"neutral-fetch-click class-nts\" data-url=\"/nts\" data-wrap=\"wrap-nts\">\n    nts...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_auto() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/url|auto| >> loading... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"\" class=\"neutral-fetch-auto \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_auto_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; |/{:;__test-nts:}|auto|wrap-{:;__test-nts:}|class-{:;__test-nts:}|id-{:;__test-nts:}| >> {:;__test-nts:}... :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div id=\"id-nts\" class=\"neutral-fetch-auto class-nts\" data-url=\"/nts\" data-wrap=\"wrap-nts\">\n    nts...\n</div>\n</div>");
    }

    #[test]
    fn test_bif_fetch_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:fetch; {:flg; invalid_flag :} '/url' >> loading... :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
