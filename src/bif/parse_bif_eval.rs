#![doc = include_str!("../../doc/bif-eval.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:eval; code >> ... {:;__eval__:} ... :} {:* embbedding *:}
    */
    pub(crate) fn parse_bif_eval(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(false);

        if self.params.contains("{:flg;") {
            return Err(BifError {
                msg: "flags not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        if self.params.contains(BIF_OPEN) {
            self.params = new_child_parse!(self, &self.params, self.mod_scope);
        }

        if (self.params != EMPTY_STRING) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                let restore_eval = self.get_data("__eval__");
                self.set_data("__eval__", &self.params.clone());
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
                self.set_data("__eval__", &restore_eval);
            }
            self.out = self.code.clone();
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:eval; a >> b :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>b</div>");
    }

    #[test]
    fn test_bif_eval_evaluation() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:eval; {:;__test-nts:} >> {:;__eval__:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_eval_negate_1() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!eval; {:;__test-nts:} >> nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_eval_negate_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!eval; {:;__test-empty-nts:} >> nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_eval_scope_1() {
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
        "<div>{:eval; {:include; tests/snippets.ntpl :} >> nts :}{:snippet; test-snippet :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_eval_scope_2() {
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
        "<div>{:+eval; {:include; tests/snippets.ntpl :} >> nts :}{:snippet; test-snippet :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_eval_scope_3() {
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
        "<div>{:+eval; 1 >> {:include; tests/snippets.ntpl :}  :}{:snippet; test-snippet :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_eval_scope_4() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+eval; {:include; tests/snippets.ntpl :} >> {:include; tests/snippets.ntpl :}  :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_eval_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:eval; {:flg; invalid_flag :} a >> b :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
