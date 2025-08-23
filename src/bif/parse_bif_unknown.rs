
use crate::{
    bif::Bif,
    bif::BifError,
};

impl<'a> Bif<'a> {
    /*
        unknown bif
    */
    pub(crate) fn parse_bif_unknown(&mut self) -> Result<(), BifError> {
        self.alias = "unknown".to_string();

        Err(BifError {
            msg: "unknown bif".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_unknown() {
        let schema = r#"
        {
            "config": {
                "comments": "keep"
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
        template.set_src_str("<div>{:unk;:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
