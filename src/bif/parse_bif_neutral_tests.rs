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
