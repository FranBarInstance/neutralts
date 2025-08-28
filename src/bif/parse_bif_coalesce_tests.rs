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
