#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_rand() {
        pub fn number(s: &str, x: usize) -> bool {
            s.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .len()
                == x
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
        template.set_src_str("{:rand; :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(number(&result, 9));
    }

    #[test]
    fn test_bif_rand_10_99() {
        pub fn number(s: &str, x: usize) -> bool {
            s.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .len()
                == x
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
        template.set_src_str("{:rand; 10..99 :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(number(&result, 2));
    }
}
