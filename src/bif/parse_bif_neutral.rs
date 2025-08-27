#![doc = include_str!("../../doc/bif-neutral.md")]

use crate::{
    bif::Bif,
    bif::BifError,
    bif::constants::*,
};

/*
    {:neutral; ... :}
*/

impl<'a> Bif<'a> {
    pub(crate) fn parse_bif_neutral(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.out = self.raw.to_string();

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_neutral() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:neutral; template >> system :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>{:neutral; template >> system :}</div>");
    }
}
