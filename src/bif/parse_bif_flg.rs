#![doc = include_str!("../../doc/bif-flg.md")]

use crate::{bif::Bif, bif::BifError, constants::*};

impl<'a> Bif<'a> {
    /*
        {:flg; flag-name1 flag-name2 ... :}
        {:code; {:flg; safe :} >>  <div>...</div> :}
    */
    pub(crate) fn parse_bif_flg(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_upline || self.mod_scope {
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

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, false);
        }

        let flags = format!(" {} ", self.code);
        self.shared.flags = flags.replace(" ", "|");
        self.out = EMPTY_STRING;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_flg() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:flg; any :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_flg_no_flags() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:flg; {:flg; any :} any >> :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
