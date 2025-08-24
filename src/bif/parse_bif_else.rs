#![doc = include_str!("../../doc/bif-else.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
       {:else; ... :}
       {:code; :}{:else; this is output :}
       {:code; not empty :}{:!else; this is output :}
    */
    pub(crate) fn parse_bif_else(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
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

        if self.inherit.last_bif_out ^ self.mod_negate {
            self.out = EMPTY_STRING;

            return Ok(());
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.out = self.code.to_string();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_else() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; :}{:else; nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_else_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; 1 :}{:!else; nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>1nts</div>");
    }

    #[test]
    fn test_bif_else_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; :}{:+else; __test-obj-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_else_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; :}{:else; __test-obj-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
