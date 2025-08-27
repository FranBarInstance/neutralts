#![doc = include_str!("../../doc/bif-hash.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};
use md5::{Digest, Md5};
use rand::Rng;

impl<'a> Bif<'a> {
    /*
        {:hash;  :}
        {:hash; text :}
    */
    pub(crate) fn parse_bif_hash(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.code = self.src.trim().to_string();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.code.trim().to_string();
        }

        if self.code.is_empty() {
            let mut hasher = Md5::new();
            let mut rng = rand::rng();
            let rand = rng.random_range(100000000..=999999999).to_string();
            hasher.update(&rand);
            self.out = format!("{:x}", hasher.finalize())
        } else {
            let mut hasher = Md5::new();
            hasher.update(&self.code);
            self.out = format!("{:x}", hasher.finalize());
        }

        Ok(())
    }
}

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
