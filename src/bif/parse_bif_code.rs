#![doc = include_str!("../../doc/bif-code.md")]

use crate::{bif::Bif, bif::BifError, constants::*, utils::*};

impl<'a> Bif<'a> {
    /*
        {:code; ...  :}
        {:code; {:flags; safe noparse encode_tags encode_tags_after encode_bifs :} >>  <div>...</div>  :}
    */
    pub(crate) fn parse_bif_code(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

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
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_code_empty() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_code_literal() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; Hello :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello</div>");
    }

    #[test]
    fn test_bif_code_evaluation() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_code_flag_safe() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:flg; safe :} >> <div>{:;__test-nts:}</div> :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>&lt;div&gt;&#123;:;__test-nts:&#125;&lt;&#x2F;div&gt;</div>"
        );
    }

    #[test]
    fn test_bif_code_flag_encode_tags() {
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
            "<div>{:code; {:flg; encode_tags :} >> <div>{:;__test-nts:}</div> :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&lt;div&gt;nts&lt;&#x2F;div&gt;</div>");
    }

    #[test]
    fn test_bif_code_flag_noparse() {
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
            .set_src_str("<div>{:code; {:flg; noparse :} >> <div>{:;__test-nts:}</div> :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>{:;__test-nts:}</div></div>");
    }

    #[test]
    fn test_bif_code_flag_encode_tags_after() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:flg; encode_tags_after :} >> <div>{:code; <div>{:;__test-nts:}</div> :}</div> :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>&lt;div&gt;&lt;div&gt;nts&lt;&#x2F;div&gt;&lt;&#x2F;div&gt;</div>"
        );
    }

    #[test]
    fn test_bif_code_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:include; {:flg; require :} >> tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_code_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+code; {:include; {:flg; require :} >> tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }
}
