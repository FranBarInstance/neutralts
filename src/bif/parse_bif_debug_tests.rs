#[cfg(test)]
mod tests {
    use crate::test_helpers::*;
    use std::fs;

    #[test]
    fn test_bif_debug_disabled() {
        let schema = r#"
        {
            "config": {
                "debug_file": ""
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
        template.set_src_str("<div>{:debug; data :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_debug_enabled() {
        let schema = r#"
        {
            "config": {
                "debug_expire": 10,
                "debug_file": "/tmp/enable-neutral-debug-8ndmdj76gals33-A"
            },
            "data": {
                "test_debug": "true"
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
        fs::File::create("/tmp/enable-neutral-debug-8ndmdj76gals33-A")
            .expect("Failed to create debug file for test");
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:debug; data->test_debug :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>\"true\"</div>");
    }

    #[test]
    fn test_bif_debug_filter() {
        let schema = r#"
        {
            "config": {
                "debug_expire": 10,
                "debug_file": "/tmp/enable-neutral-debug-8ndmdj76gals33-A"
            },
            "data": {
                "test_debug": "true",
                "code": "<div>{:;test_debug:}</div>"
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
        fs::File::create("/tmp/enable-neutral-debug-8ndmdj76gals33-A")
            .expect("Failed to create debug file for test");
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:&debug; data->code :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>&quot;&lt;div&gt;&#123;:;test_debug:&#125;&lt;&#x2F;div&gt;&quot;</div>"
        );
    }

    #[test]
    fn test_bif_debug_no_filter() {
        let schema = r#"
        {
            "config": {
                "debug_expire": 10,
                "debug_file": "/tmp/enable-neutral-debug-8ndmdj76gals33-A"
            },
            "data": {
                "test_debug": "true",
                "code": "<div>{:;test_debug:}</div>"
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
        fs::File::create("/tmp/enable-neutral-debug-8ndmdj76gals33-A")
            .expect("Failed to create debug file for test");
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:debug; data->code :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>\"<div>{:;test_debug:}</div>\"</div>");
    }

    #[test]
    fn test_bif_debug_expire() {
        let schema = r#"
        {
            "config": {
                "debug_expire": 0,
                "debug_file": "/tmp/enable-neutral-debug-8ndmdj76gals33-B"
            },
            "data": {
                "test_debug": "true"
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
        fs::File::create("/tmp/enable-neutral-debug-8ndmdj76gals33-B")
            .expect("Failed to create debug file for test");
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:debug; data->test_debug :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_debug_no_debug_enabled() {
        let schema = r#"
        {
            "config": {
                "debug_expire": 3600,
                "debug_file": "/tmp/enable-neutral-debug-8ndmdj76gals33-B"
            },
            "data": {
                "test_debug": "true"
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
        fs::File::create("/tmp/enable-neutral-debug-8ndmdj76gals33-B")
            .expect("Failed to create debug file for test");
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:!debug; data->test_debug :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_debug_no_debug_disabled() {
        let schema = r#"
        {
            "config": {
                "debug_file": ""
            },
            "data": {
                "test_debug": "true"
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
        template.set_src_str("<div>{:!debug; data->test_debug :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
