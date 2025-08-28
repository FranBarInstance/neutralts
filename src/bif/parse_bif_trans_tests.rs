#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_trans() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
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
        template.set_src_str("<div>{:trans; Hello nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Γεια σας</div>");
    }

    #[test]
    fn test_bif_trans_dynamic_evaluation() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
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
        template.set_src_str("<div>{:trans; {:;__hello-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Γεια σας</div>");
    }

    #[test]
    fn test_bif_trans_no_trans() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
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
        template.set_src_str("<div>{:trans; This text has no __translation__ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>This text has no __translation__</div>");
    }

    #[test]
    fn test_bif_trans_no_trans_negate() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
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
        template.set_src_str("<div>{:!trans; This text has no __translation__ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_trans_negate() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
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
        template.set_src_str("<div>{:!trans; Hello nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Γεια σας</div>");
    }

    #[test]
    fn test_bif_trans_error_scope() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "el"
            }
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
        template.set_src_str("<div>{:+trans; Hello nts :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
