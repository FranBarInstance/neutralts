#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_locale() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str("<div>{:locale; tests/locale.es.json :}{:trans; test-locale :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_locale_evaluation() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str(
            "<div>{:locale; tests/locale.{:lang;:}.json :}{:trans; Test {:;__test-nts:} :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok nts</div>");
    }

    #[test]
    fn test_bif_locale_flag_inline() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
        .trim();
        let source = r#"
        {:locale; {:flg; inline :} >>
            {
                "trans": {
                    "{:lang;:}": {
                        "test-locale": "inline"
                    }
                }
            }
        :}
        <div>{:trans; test-locale :}</div>
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
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>inline</div>");
    }

    #[test]
    fn test_bif_locale_flag_require() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str(
        "<div>{:locale; {:flg; require :} >> tests/locale.es.json :}{:trans; test-locale :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_locale_flag_require_fails() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template
            .set_src_str("<div>{:locale; {:flg; require :} >> tests/not-found.es.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_not_found() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template
            .set_src_str("<div>{:locale; tests/not-found.es.json :}{:trans; test-locale :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>test-locale</div>");
    }

    #[test]
    fn test_bif_locale_allow() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str("<div>{:locale; {:allow; any >> {:;__test-nts:} :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_allow_fails() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str("<div>{:locale; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_negate() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str("<div>{:!locale; tests/locale.es.json :}{:!locale; tests/locale.es.json :}{:trans; test-locale :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_locale_scope() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template.set_src_str("<div>{:+locale; tests/locale.es.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_invalid_flag() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
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
        template
            .set_src_str("<div>{:locale; {:flg; invalid_flag :} >> tests/locale.es.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
