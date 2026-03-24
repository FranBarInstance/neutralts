#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_each() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:each; __test-obj-nts->level1-obj->level2-obj->level3-arr key value >> {:;key:}={:;value:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0=one1=two2=three</div>");
    }

    #[test]
    fn test_bif_each_iterate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:code; {:param; array-name >> __test-obj-nts :} {:snippet; iterate-array :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
        result,
        "<div>\n        level1=Ok\n        level1-obj:\n                level1=Ok\n                level2-obj:\n                        level2=Ok\n                        level3-arr:\n                                0=one\n                                1=two\n                                2=three</div>"
    );
    }

    #[test]
    fn test_bif_each_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:each; __test-arr-nts key val >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_each_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+each; __test-arr-nts key val >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_each_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:each; {:flg; invalid_flag :} __test-obj-nts->level1-obj->level2-obj->level3-arr key value >> {:;key:}={:;value:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
    #[test]
    fn test_bif_each_restore_value() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        
        // Loop using __test-obj-nts as the iterator value
        template.set_src_str("<div>{:each; __test-arr-nts key __test-obj-nts >> :} __test-obj-nts->level1: {:; __test-obj-nts->level1 :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        
        // After the each block, __test-obj-nts should be restored and its level1 property should be "Ok"
        assert_eq!(result, "<div> __test-obj-nts->level1: Ok</div>");
    }

    #[test]
    fn test_bif_each_local_error() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        
        template.set_src_str("<div>{:each; __test-arr-nts local::key val >> {:;val:} :}</div>");
        let result1 = template.render();
        assert!(template.has_error());
        assert_eq!(result1, "<div></div>");
        
        // Let's reset the error state and test the second case: key local::val
        let mut template2 = crate::Template::new().unwrap();
        template2.merge_schema_str(SCHEMA).unwrap();
        
        template2.set_src_str("<div>{:each; __test-arr-nts key local::val >> {:;val:} :}</div>");
        let result2 = template2.render();
        assert!(template2.has_error());
        assert_eq!(result2, "<div></div>");
    }
}
