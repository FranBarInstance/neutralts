#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_join() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:join; /__test-arr-nts/|/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>one|two|three</div>");
    }

    #[test]
    fn test_bif_join_empty() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:join; /array->empty/|/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_join_no_array() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:join; /text/|/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_join_keys_true() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:join; /__test-arr-nts/|/true/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0|1|2</div>");
    }

    #[test]
    fn test_bif_join_keys_false() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:join; /__test-arr-nts/|/false/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>one|two|three</div>");
    }

    #[test]
    fn test_bif_join_eval() {
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
            "<div>{:join; /__test-arr-nts/{:;__hello-nts:}/{:;array->zero:}/ :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>oneHello ntstwoHello ntsthree</div>");
    }

    #[test]
    fn test_bif_join_eval_true() {
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
            .set_src_str("<div>{:join; /__test-arr-nts/{:;__hello-nts:}/{:;array->one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0Hello nts1Hello nts2</div>");
    }
}
