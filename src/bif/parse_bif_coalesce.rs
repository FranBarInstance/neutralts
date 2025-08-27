#![doc = include_str!("../../doc/bif-coalesce.md")]

use crate::{bif::Bif, bif::BifError, bif::constants::*,};

impl<'a> Bif<'a> {
    /*
       {:coalesce;
           {:code;  :}
           {:code; this is output :}
           {:code; ... :}
       :}
    */
    pub(crate) fn parse_bif_coalesce(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        // This var so as not to overwrite the original: inherit.last_bif_out
        self.inherit.last_coalesce_out = false;
        self.out = new_child_parse!(self, &self.src, self.mod_scope);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_coalesce() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:coalesce; {:code; :} {:code; this :} {:code; ... :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>this</div>");
    }

    #[test]
    fn test_bif_coalesce_evaluation() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:coalesce; {:;__test-empty-nts:} {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_coalesce_nested() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:coalesce; {:code; :} {:coalesce; {:code; :} {:coalesce; {:code; :} {:code; this :} {:code; ... :} :} {:code; ... :} :} {:code; ... :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>this</div>");
    }

    #[test]
    fn test_bif_coalesce_negate_fails() {
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
            .set_src_str("<div>{:!coalesce; {:code; :} {:code; this :} {:code; ... :} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_coalesce_big() {
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
            .set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; big-coalesce :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>OK1-Ok2-Ok3-Ok4-Ok5-Ok6</div>");
    }

    #[test]
    fn test_bif_coalesce_scope() {
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
        "<div>{:+coalesce; {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_coalesce_no_scope() {
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
            "<div>{:coalesce; {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
