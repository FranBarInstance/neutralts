#![doc = include_str!("../../doc/bif-declare.md")]

use crate::{bif::Bif, bif::BifError, constants::*, json};

impl<'a> Bif<'a> {
    /*
        {:declare; name >> words list :}
    */
    pub(crate) fn parse_bif_declare(&mut self) -> Result<(), BifError> {
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

        if self.inherit.current_file.contains(SNIPPETS_FILES) {
            self.inherit.create_block_schema(self.shared);
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
                self.code = self.code.replace(UNPRINTABLE, "");
            }
            self.shared.schema["__indir"][&self.inherit.indir]["declare"][&self.params] =
                json!(&self.code);

            self.out = EMPTY_STRING;
        } else {
            return Err(BifError {
                msg: "declare cannot be set here".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_declare() {
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
            "<div>{:include; tests/snippets.ntpl :}{:allow; test-for-tests >> one :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>one</div>");
    }

    #[test]
    fn test_bif_declare_no_outside_include() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:declare; test-for-tests >> one two three :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_declare_invalid_flag() {
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
            "<div>{:include; tests/snippets-declare-invalid-flag.ntpl :}{:allow; test-for-tests >> one :}</div>",
        );
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
