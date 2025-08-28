#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_sum() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /1/2/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>3</div>");
    }

    #[test]
    fn test_bif_sum_decimals() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /1.5/2.1/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>3.6</div>");
    }

    #[test]
    fn test_bif_sum_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /{:;one:}/{:;one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>2</div>");
    }

    #[test]
    fn test_bif_sum_nested() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /{:;one:}/{:sum;|{:;one:}|{:;one:}|:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>3</div>");
    }

    #[test]
    fn test_bif_sum_subtract() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /{:;one:}/-{:;one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0</div>");
    }

    #[test]
    fn test_bif_sum_subtract_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /-{:;one:}/{:;one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0</div>");
    }
}
