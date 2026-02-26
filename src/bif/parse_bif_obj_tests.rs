#[cfg(test)]
mod tests {
    use crate::test_helpers::*;
    use std::env;
    use std::fs;
    use std::net::{TcpStream, ToSocketAddrs};
    use std::os::unix::net::UnixStream;
    use std::path::Path;
    use std::process;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn set_php_src(template: &mut crate::Template, src: &str, fpm_endpoint: &str) {
        template.set_src_str(&src.replace("__FPM__", fpm_endpoint));
    }

    fn set_php_src_with_script(
        template: &mut crate::Template,
        src: &str,
        script_path: &str,
        fpm_endpoint: &str,
    ) {
        let src = src
            .replace("__SCRIPT__", script_path)
            .replace("__FPM__", fpm_endpoint);
        template.set_src_str(&src);
    }

    fn set_python_src_with_script(template: &mut crate::Template, src: &str, script_path: &str) {
        template.set_src_str(&src.replace("__SCRIPT__", script_path));
    }

    fn can_connect_php_fpm(endpoint: &str) -> bool {
        if let Some(path) = endpoint.strip_prefix("unix:") {
            return UnixStream::connect(path).is_ok();
        }

        if let Some(address) = endpoint.strip_prefix("tcp://") {
            let Ok(mut addrs) = address.to_socket_addrs() else {
                return false;
            };
            let Some(addr) = addrs.next() else {
                return false;
            };
            return TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok();
        }

        false
    }

    fn default_obj_php_fpm() -> String {
        serde_json::from_str::<crate::Value>(crate::DEFAULT)
            .ok()
            .and_then(|schema| {
                schema["config"]["obj_php_fpm"]
                    .as_str()
                    .map(|v| v.to_string())
            })
            .unwrap_or_else(|| "unix:/run/php/php-fpm.sock".to_string())
    }

    fn php_fpm_test_endpoint() -> Option<String> {
        if let Ok(value) = env::var("NEUTRALTS_TEST_PHP_FPM") {
            let value = value.trim();
            if value.is_empty() || value.eq_ignore_ascii_case("skip") {
                return None;
            }
            if can_connect_php_fpm(value) {
                return Some(value.to_string());
            }
            return None;
        }

        let default_endpoint = default_obj_php_fpm();

        if let Some(path) = default_endpoint.strip_prefix("unix:") {
            if Path::new(path).exists() && can_connect_php_fpm(&default_endpoint) {
                return Some(default_endpoint);
            }
            return None;
        }

        if can_connect_php_fpm(&default_endpoint) {
            return Some(default_endpoint);
        }
        None
    }

    fn php_fpm_test_endpoint_or_skip(test_name: &str) -> Option<String> {
        match php_fpm_test_endpoint() {
            Some(endpoint) => Some(endpoint),
            None => {
                eprintln!(
                    "[SKIP] {}: PHP-FPM not available. Set NEUTRALTS_TEST_PHP_FPM or ensure {} exists.",
                    test_name,
                    default_obj_php_fpm()
                );
                None
            }
        }
    }

    fn create_php_test_script() -> String {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = format!("tests/__obj_php_{}_{}.php", process::id(), nanos);
        let script = r#"<?php
function main($params = []) {
    $schema = $GLOBALS['__NEUTRAL_SCHEMA__'] ?? null;
    $schema_data = $GLOBALS['__NEUTRAL_SCHEMA_DATA__'] ?? null;

    $schema_seen = is_array($schema) ? "yes" : "no";
    $schema_data_kind = "none";
    if (is_array($schema_data)) {
        $schema_data_kind = "dict";
    } else if (is_string($schema_data) || is_numeric($schema_data) || is_bool($schema_data)) {
        $schema_data_kind = "scalar";
    }

    return [
        "data" => [
            "php_hello" => "Hello from PHP!",
            "schema_seen" => $schema_seen,
            "schema_data_kind" => $schema_data_kind,
        ]
    ];
}
"#;
        fs::write(&path, script).unwrap();
        path
    }

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
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"tests/script.php\",\"fpm\":\"bad-endpoint\"} >> :}</div>",
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
        set_python_src_with_script(
            &mut template,
            "<div>{:obj; {\"file\":\"__SCRIPT__\",\"schema_data\":\"__test-nts\"} >> {:;local::schema_data_kind:}|{:;local::schema_data_scalar:} :}</div>",
            &script_path,
        );
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
        set_python_src_with_script(
            &mut template,
            "<div>{:obj; {\"file\":\"__SCRIPT__\",\"schema_data\":\"__test-arr-nts\"} >> {:;local::schema_data_kind:} :}</div>",
            &script_path,
        );
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
        set_python_src_with_script(
            &mut template,
            "<div>{:obj; {\"file\":\"__SCRIPT__\",\"schema_data\":\"__test-obj-nts->level1-obj->level2-obj->level2\"} >> {:;local::schema_data_kind:}|{:;local::schema_data_scalar:} :}</div>",
            &script_path,
        );
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
        set_python_src_with_script(
            &mut template,
            "<div>{:data; tests/local-data.json :}{:obj; {\"file\":\"__SCRIPT__\",\"schema_data\":\"local::array\"} >> {:;local::schema_data_kind:} :}</div>",
            &script_path,
        );
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
        set_python_src_with_script(
            &mut template,
            "<div>{:data; tests/local-data.json :}{:obj; {\"file\":\"__SCRIPT__\",\"schema_data\":\"local::nested-obj->Lorem->Ipsum->Dolor->Sit->Amet\"} >> {:;local::schema_data_kind:}|{:;local::schema_data_scalar:} :}</div>",
            &script_path,
        );
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
        set_python_src_with_script(
            &mut template,
            "<div>{:obj; {\"file\":\"__SCRIPT__\",\"schema_data\":\"local::missing-key\",\"schema\":true} >> {:;local::schema_data_kind:}|{:;local::schema_present:} :}</div>",
            &script_path,
        );
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>none|true</div>");
    }

    #[test]
    fn test_bif_obj_php_exec_conditional() {
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php_exec_conditional")
        else {
            return;
        };

        let script_path = create_php_test_script();
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
        set_php_src_with_script(
            &mut template,
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"__SCRIPT__\",\"fpm\":\"__FPM__\",\"schema\":true,\"schema_data\":\"__test-nts\"} >> {:;local::php_hello:}|{:;local::schema_seen:}|{:;local::schema_data_kind:} :}</div>",
            &script_path,
            &fpm_endpoint,
        );
        let result = template.render();
        remove_test_script(&script_path);

        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from PHP!|yes|scalar</div>");
    }

    #[test]
    fn test_bif_obj_php() {
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php") else {
            return;
        };

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        set_php_src(
            &mut template,
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"tests/script.php\",\"fpm\":\"__FPM__\"} >> {:;local::php_hello:} :}</div>",
            &fpm_endpoint,
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from PHP!</div>");
    }

    #[test]
    fn test_bif_obj_php_no_scope() {
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php_no_scope") else {
            return;
        };

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        set_php_src(
            &mut template,
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"tests/script.php\",\"fpm\":\"__FPM__\"} >>  :}{:;local::php_hello:}</div>",
            &fpm_endpoint,
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_php_scope() {
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php_scope") else {
            return;
        };

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        set_php_src(
            &mut template,
            "<div>{:+obj; {\"engine\":\"php\",\"file\":\"tests/script.php\",\"fpm\":\"__FPM__\"} >>  :}{:;local::php_hello:}</div>",
            &fpm_endpoint,
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello from PHP!</div>");
    }

    #[test]
    fn test_bif_obj_php_schema_false() {
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php_schema_false")
        else {
            return;
        };

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        set_php_src(
            &mut template,
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"tests/script.php\",\"fpm\":\"__FPM__\",\"schema\":false} >> {:;local::test_nts:} :}</div>",
            &fpm_endpoint,
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_obj_php_schema_true() {
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php_schema_true")
        else {
            return;
        };

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        set_php_src(
            &mut template,
            "<div>{:obj; {\"engine\":\"php\",\"file\":\"tests/script.php\",\"fpm\":\"__FPM__\",\"schema\":true} >> {:;local::test_nts:} :}</div>",
            &fpm_endpoint,
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_obj_php_with_params() {
        // let Some(fpm_endpoint) = php_fpm_test_endpoint() else {
        //     return;
        // };
        let Some(fpm_endpoint) = php_fpm_test_endpoint_or_skip("test_bif_obj_php_with_params")
        else {
            return;
        };

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        template.merge_schema_str(SCHEMA).unwrap();
        set_php_src(
            &mut template,
            "<div>{:obj; { \"engine\":\"php\", \"file\": \"tests/script.php\", \"fpm\":\"__FPM__\", \"params\": {\"param1\":\"{:;__test-nts:}\"} } >> {:;local::param1:} :}</div>",
            &fpm_endpoint,
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }
}
