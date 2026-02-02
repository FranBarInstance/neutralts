#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_obj() {
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
            "<div>{:obj; { \"file\": \"tests/script.py\" } >> {:;local::py_hello:} :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_no_scope() {
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
            "<div>{:obj; { \"file\": \"tests/script.py\" } >>  :}{:;local::py_hello:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_scope() {
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
            "<div>{:+obj; { \"file\": \"tests/script.py\" } >>  :}{:;local::py_hello:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_objfile() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; tests/obj.json :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from Python!</div>");
    }

    #[test]
    fn test_bif_obj_invalid_file() {
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
            "<div>{:obj; { \"file\": \"nonexistent.py\" } >> {:;local::py_hello:} :}</div>",
        );
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:flg; unknown :} { \"file\": \"tests/script.py\" } >> {:;local::py_hello:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_insecure_filename() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {:;dangerous:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_invalid_json() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {malformed json} >> :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_unsupported_engine() {
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
            "<div>{:obj; {\"engine\":\"ruby\",\"file\":\"tests/script.py\"} >> :}</div>",
        );
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_schema_false() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {\"file\":\"tests/script.py\",\"schema\":false} >> {:;local::test_nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    // https://github.com/FranBarInstance/neutralts/issues/2
    #[test]
    fn test_bif_obj_schema_true_first() {
        test_bif_obj_schema_true();
        test_bif_obj_schema_false();
    }

    #[test]
    fn test_bif_obj_schema_true() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; {\"file\":\"tests/script.py\",\"schema\":true} >> {:;local::test_nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_obj_missing_obj_file() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; nonexistent.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_template_integration() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; tests/obj.json >> |Inline:{:;local::py_hello:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>Hello from Python!|Inline:Hello from Python!</div>"
        );
    }

    #[test]
    fn test_bif_obj_with_params() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:obj; { \"file\": \"tests/script.py\", \"params\": {\"param1\":\"{:;__test-nts:}\"} } >> {:;local::param1:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }
}
