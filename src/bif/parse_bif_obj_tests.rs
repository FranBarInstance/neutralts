#[cfg(test)]
mod tests {
    use crate::test_helpers::*;
    use std::fs;
    use std::process;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_schema_data_test_script() -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = format!("tests/__obj_schema_data_{}_{}.py", process::id(), nanos);
        let script = r#"def main(params=None):
    data = globals().get('__NEUTRAL_SCHEMA_DATA__', None)
    if data is None:
        kind = 'none'
        scalar = ''
    elif isinstance(data, dict):
        kind = 'dict'
        scalar = ''
    elif isinstance(data, list):
        kind = 'list'
        scalar = ''
    else:
        kind = 'scalar'
        scalar = str(data)

    schema_present = '__NEUTRAL_SCHEMA__' in globals()
    return {
        'data': {
            'schema_data_kind': kind,
            'schema_data_scalar': scalar,
            'schema_present': schema_present
        }
    }
"#;
        fs::write(&path, script).unwrap();
        path
    }

    fn remove_test_script(path: &str) {
        let _ = fs::remove_file(path);
    }

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
    fn test_bif_obj_php_invalid_fpm_endpoint() {
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
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"tests/script.py\",\"fpm\":\"bad-endpoint\"} >> :}</div>",
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

    #[test]
    fn test_bif_obj_schema_data_scalar() {
        let script_path = create_schema_data_test_script();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                remove_test_script(&script_path);
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(&format!(
            "<div>{{:obj; {{\"file\":\"{}\",\"schema_data\":\"__test-nts\"}} >> {{:;local::schema_data_kind:}}|{{:;local::schema_data_scalar:}} :}}</div>",
            script_path
        ));
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>scalar|nts</div>");
    }

    #[test]
    fn test_bif_obj_schema_data_list_global() {
        let script_path = create_schema_data_test_script();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                remove_test_script(&script_path);
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(&format!(
            "<div>{{:obj; {{\"file\":\"{}\",\"schema_data\":\"__test-arr-nts\"}} >> {{:;local::schema_data_kind:}} :}}</div>",
            script_path
        ));
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>list</div>");
    }

    #[test]
    fn test_bif_obj_schema_data_nested_global_scalar() {
        let script_path = create_schema_data_test_script();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                remove_test_script(&script_path);
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(&format!(
            "<div>{{:obj; {{\"file\":\"{}\",\"schema_data\":\"__test-obj-nts->level1-obj->level2-obj->level2\"}} >> {{:;local::schema_data_kind:}}|{{:;local::schema_data_scalar:}} :}}</div>",
            script_path
        ));
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>scalar|Ok</div>");
    }

    #[test]
    fn test_bif_obj_schema_data_dict_local() {
        let script_path = create_schema_data_test_script();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                remove_test_script(&script_path);
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(&format!(
            "<div>{{:data; tests/local-data.json :}}{{:obj; {{\"file\":\"{}\",\"schema_data\":\"local::array\"}} >> {{:;local::schema_data_kind:}} :}}</div>",
            script_path
        ));
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>dict</div>");
    }

    #[test]
    fn test_bif_obj_schema_data_nested_local_scalar() {
        let script_path = create_schema_data_test_script();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                remove_test_script(&script_path);
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(&format!(
            "<div>{{:data; tests/local-data.json :}}{{:obj; {{\"file\":\"{}\",\"schema_data\":\"local::nested-obj->Lorem->Ipsum->Dolor->Sit->Amet\"}} >> {{:;local::schema_data_kind:}}|{{:;local::schema_data_scalar:}} :}}</div>",
            script_path
        ));
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>scalar|Consectetur adipiscing elit.</div>");
    }

    #[test]
    fn test_bif_obj_schema_data_missing_is_none() {
        let script_path = create_schema_data_test_script();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                remove_test_script(&script_path);
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(&format!(
            "<div>{{:obj; {{\"file\":\"{}\",\"schema_data\":\"local::missing-key\",\"schema\":true}} >> {{:;local::schema_data_kind:}}|{{:;local::schema_present:}} :}}</div>",
            script_path
        ));
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>none|true</div>");
    }
}
