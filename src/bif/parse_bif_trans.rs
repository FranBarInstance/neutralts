#![doc = include_str!("../../doc/bif-trans.md")]

use crate::{bif::Bif, bif::BifError, constants::*, bif::constants::*,};

impl<'a> Bif<'a> {
    /*
       {:trans; ... :}
    */
    pub(crate) fn parse_bif_trans(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        // For performance, we avoid calling BlockParser::new if it is not necessary
        if self.src.contains(BIF_OPEN) {
            self.src = new_child_parse!(self, &self.src, self.mod_scope);
        }

        let trans = self.get_trans(&self.src);

        // By default the input text
        if trans.is_empty() {
            if self.mod_negate {
                self.out = EMPTY_STRING;
            } else {
                self.out = self.src.clone();
            }
        } else {
            self.out = trans;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_trans() {
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
        template.set_src_str("<div>{:trans; Hello nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Γεια σας</div>");
    }

    #[test]
    fn test_bif_trans_dynamic_evaluation() {
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
        template.set_src_str("<div>{:trans; {:;__hello-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Γεια σας</div>");
    }

    #[test]
    fn test_bif_trans_no_trans() {
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
        template.set_src_str("<div>{:trans; This text has no __translation__ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>This text has no __translation__</div>");
    }

    #[test]
    fn test_bif_trans_no_trans_negate() {
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
        template.set_src_str("<div>{:!trans; This text has no __translation__ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_trans_negate() {
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
        template.set_src_str("<div>{:!trans; Hello nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Γεια σας</div>");
    }

    #[test]
    fn test_bif_trans_error_scope() {
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
        template.set_src_str("<div>{:+trans; Hello nts :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
