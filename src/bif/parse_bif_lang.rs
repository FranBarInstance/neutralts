#![doc = include_str!("../../doc/bif-lang.md")]

use crate::{bif::Bif, bif::BifError, bif::constants::*,};

impl<'a> Bif<'a> {
    /*
       {:lang; ... :}
    */
    pub(crate) fn parse_bif_lang(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.out = self.shared.lang.to_string();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_lang() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
        .trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:lang;:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>el</div>");
    }

    #[test]
    fn test_bif_lang_comment() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
        .trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:lang; {:* comment *:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>el</div>");
    }

    #[test]
    fn test_bif_lang_error_negate() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
        .trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:!lang;:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_lang_error_scope() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
        }
    }
    "#
        .trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:+lang;:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
