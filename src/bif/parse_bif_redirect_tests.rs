#[cfg(test)]
mod tests {
    use crate::constants::*;
    use crate::test_helpers::*;

    #[test]
    fn test_bif_redirect_301() {
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
            "<div>{:;__test-nts:}{:redirect; 301 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "301");
        assert_eq!(template.get_status_text(), "Moved Permanently");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "301 Moved Permanently\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_302() {
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
            "<div>{:;__test-nts:}{:redirect; 302 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "302");
        assert_eq!(template.get_status_text(), "Found");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "302 Found\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_303() {
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
            "<div>{:;__test-nts:}{:redirect; 303 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "303");
        assert_eq!(template.get_status_text(), "See Other");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "303 See Other\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_307() {
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
            "<div>{:;__test-nts:}{:redirect; 307 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "307");
        assert_eq!(template.get_status_text(), "Temporary Redirect");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "307 Temporary Redirect\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_308() {
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
            "<div>{:;__test-nts:}{:redirect; 308 >> https://example.com/ :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "308");
        assert_eq!(template.get_status_text(), "Permanent Redirect");
        assert_eq!(template.get_status_param(), "https://example.com/");
        assert_eq!(result, "308 Permanent Redirect\nhttps://example.com/");
    }

    #[test]
    fn test_bif_redirect_js_reload_top() {
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
            .set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:top :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "js:reload:top");
        assert_eq!(result, REDIR_JS_RELOAD_TOP);
    }

    #[test]
    fn test_bif_redirect_js_reload_top_param() {
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
            "<div>{:;__test-nts:}{:redirect; js:reload:top >> some :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "some");
        assert_eq!(result, REDIR_JS_RELOAD_TOP);
    }

    #[test]
    fn test_bif_redirect_js_reload_self() {
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
            .set_src_str("<div>{:;__test-nts:}{:redirect; js:reload:self :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "js:reload:self");
        assert_eq!(result, REDIR_JS_RELOAD_SELF);
    }

    #[test]
    fn test_bif_redirect_js_reload_self_param() {
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
            "<div>{:;__test-nts:}{:redirect; js:reload:self >> some :}{:;__test-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "some");
        assert_eq!(result, REDIR_JS_RELOAD_SELF);
    }

    #[test]
    fn test_bif_redirect_negate_fails() {
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
            .set_src_str("<div>{:;__test-nts:}{:!redirect; js:reload:top :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_redirect_fails_no_params_1() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:!redirect; :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_redirect_fails_no_params_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}{:!redirect; 301 >> :}{:;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(template.get_status_code(), "200");
        assert_eq!(template.get_status_text(), "OK");
        assert_eq!(template.get_status_param(), "");
        assert_eq!(result, "<div>ntsnts</div>");
    }

    #[test]
    fn test_bif_redirect_parse_url() {
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
            "<div>{:redirect; 301 >> https://example.com/?{:;__test-nts:} :}/div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(template.get_status_code(), "301");
        assert_eq!(template.get_status_text(), "Moved Permanently");
        assert_eq!(template.get_status_param(), "https://example.com/?nts");
        assert_eq!(result, "301 Moved Permanently\nhttps://example.com/?nts");
    }

}
