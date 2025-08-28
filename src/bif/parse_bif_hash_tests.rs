#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_hash() {
        fn is_md5_like(s: &str) -> bool {
            s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit())
        }

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("{:hash; :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(is_md5_like(&result));
    }

    #[test]
    fn test_bif_hash_evaluate() {
        fn is_md5_like(s: &str) -> bool {
            s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit())
        }

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("{:hash; {:;__hello-nts:} :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(is_md5_like(&result));
    }

    #[test]
    fn test_bif_hash_evaluate_2() {
        use md5::{Digest, Md5};

        pub fn calculate_md5(s: &str) -> String {
            let mut hasher = Md5::new();
            hasher.update(s);
            format!("{:x}", hasher.finalize())
        }
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("{:hash; {:;__test-nts:} :}");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(calculate_md5("nts"), result);
    }
}
