#![doc = include_str!("../../doc/bif-param.md")]

use crate::{bif::Bif, bif::BifError, constants::*, json, utils::*};

impl<'a> Bif<'a> {
    /*
        Play param: {:param; param-name :}
        Set param:  {:param; param-name >> content to set :}
    */
    pub(crate) fn parse_bif_param(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let is_set = self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(BifError {
                msg: "flags not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        if is_set {
            if self.inherit.alias == "code" {
                if self.code.contains(BIF_OPEN) {
                    self.code = new_child_parse!(self, &self.code, self.mod_scope);
                }

                self.inherit.create_block_schema(self.shared);
                self.shared.schema["__indir"][&self.inherit.indir]["params"][&self.params] =
                    json!(&self.code);
                self.out = EMPTY_STRING;

                Ok(())
            } else {
                Err(BifError {
                    msg: "param cannot be set here".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        } else {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }

            self.code = get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["params"],
                &self.code,
            );
            self.out = self.code.to_string();

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_param() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:param; 1 >> one :} {:param; 1 :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>one</div>");
    }

    #[test]
    fn test_bif_param_set_outside_code() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:param; 1 >> one :}{:param; 1 :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_param_evaluation() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:param; {:;__test-nts:} >> {:;__test-nts:} :} {:param; {:;__test-nts:} :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_param_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:param; 1 >> one :} {:code; {:param; 1 :} :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>one</div>");
    }

    #[test]
    fn test_bif_param_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:param; {:;__test-nts:} >> {:;__test-nts:} :} :}{:param; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_param_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:code; {:param; {:flg; invalid_flag :} {:;__test-nts:} >> {:;__test-nts:} :} :}{:param; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
