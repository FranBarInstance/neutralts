#![doc = include_str!("../../doc/bif-same.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:same; /a/b/ >> ... :}
    */
    pub(crate) fn parse_bif_same(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(false);
        let args = self.extract_args();

        let param1 = args.get(1).cloned().ok_or_else(|| BifError {
            msg: "arguments not found".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        let param2 = args.get(2).cloned().ok_or_else(|| BifError {
            msg: "arguments not found".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        if (param1 == param2) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
            self.out = self.code.to_string();
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
    fn test_bif_same() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:same; /a/a/ >> is same :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is same</div>");
    }

    #[test]
    fn test_bif_no_same() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:same; /a/b/ >> is same :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_no_same_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!same; /a/b/ >> not same :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>not same</div>");
    }

    #[test]
    fn test_bif_same_evaluate() {
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
            "<div>{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }
}
