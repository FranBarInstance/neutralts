#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_replace() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:replace; /a/b/ >> acbde :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>bcbde</div>");
    }

    #[test]
    fn test_bif_replace_evaluation() {
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
            "<div>{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello one</div>");
    }

    #[test]
    fn test_bif_replace_delim_1() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:replace; |a|b| >> acbde :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>bcbde</div>");
    }

    #[test]
    fn test_bif_replace_delim_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:replace; ~a~b~ >> acbde :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>bcbde</div>");
    }

    #[test]
    fn test_bif_replace_delim_3() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:replace; :a:b: >> acbde :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>bcbde</div>");
    }

    #[test]
    fn test_bif_replace_params_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:replace; a/b >> acbde :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_replace_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:replace; {:flg; invalid_flag :} /a/b/ >> acbde :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
