#![doc = include_str!("../../doc/bif-allow.md")]

use std::collections::HashSet;
use crate::{
    bif::Bif,
    bif::BifError,
    bif::constants::*,
    constants::*,
    utils::*,
};

impl<'a> Bif<'a> {
    /*
        {:allow; {:flg; partial casein replace :} name >> ... :}
    */
    pub(crate) fn parse_bif_allow(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);
        let mut found = String::new();
        let words_string = get_from_key(
            &self.shared.schema["__indir"][&self.inherit.indir]["declare"],
            &self.params,
        );

        if words_string.is_empty() {
            return Err(self.bif_error(&(self.params.clone() + BIF_ERROR_DECLARED_IS_EMPTY)));
        }

        if !self.flags.is_empty() {
            let flags_allowed: HashSet<&str> = [
                "partial",
                "replace",
                "casein"
            ].into_iter().collect();

            for f in self.flags.split('|').filter(|s| !s.is_empty()) {
                if !flags_allowed.contains(f) {
                    return Err(self.bif_error(&format!("{} flag not allowed", f)));
                }
            }
        }

        let mut words_list: Vec<&str> = words_string.split_whitespace().collect();
        self.code = new_child_parse!(self, &self.code, self.mod_scope);

        for word in &mut words_list {
            let lower_haystack;
            let mut haystack = &self.code;
            let mut pattern = word.to_string().clone();

            if self.flags.contains("|partial|") || self.flags.contains("|replace|") {
                pattern = format!("{}{}{}", "*", pattern, "*");
            }

            if self.flags.contains("|casein|") {
                pattern = pattern.to_lowercase();
                lower_haystack = self.code.clone().to_lowercase();
                haystack = &lower_haystack;
            }

            if wildcard_match(haystack, &pattern) {
                found = word.to_string();
                break;
            }
        }

        if !found.is_empty() ^ self.mod_negate {
            if self.flags.contains("|replace|") {
                found = found.replace("~", "");
                found = found.replace("*", "");
                found = found.replace("?", "");
                found = found.replace(".", "");
                self.out = found.to_string();
            } else {
                self.out = self.code.to_string();
            }
        } else {
            self.out = EMPTY_STRING;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_allow() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts >> en :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>en</div>");
    }

    #[test]
    fn test_bif_allow_evaluate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-{:;__test-nts:} >> {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_allow_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts >> notallow :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!allow; traversal >> is not traversal :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not traversal</div>");
    }

    #[test]
    fn test_bif_allow_negate_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!allow; traversal >> ../istraversal :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_any() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; any >> something :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>something</div>");
    }

    #[test]
    fn test_bif_allow_wildcard_empty() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-empty >>  :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_wildcard_asterisk() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-asterisk >> en-nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>en-nts</div>");
    }

    #[test]
    fn test_bif_allow_wildcard_asterisk_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-asterisk >> not :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_wildcard_dot_1() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-dot >> ennts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>ennts</div>");
    }

    #[test]
    fn test_bif_allow_wildcard_dot_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-dot >> en-nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>en-nts</div>");
    }

    #[test]
    fn test_bif_allow_wildcard_dot_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-dot >> not :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_wildcard_question() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-question >> en-nts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>en-nts</div>");
    }

    #[test]
    fn test_bif_allow_wildcard_question_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; _test-nts-question >> ennts :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+allow; _test-nts >> notallow :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_allow_flag_partial() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow;{:flg; partial :} _test-nts >> nts and more :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts and more</div>");
    }

    #[test]
    fn test_bif_allow_flag_casein() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow;{:flg; casein :} _test-nts >> NTS :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>NTS</div>");
    }

    #[test]
    fn test_bif_allow_flag_replace() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow;{:flg; replace :} _test-nts >> nts and more :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_allow_multi_flags() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; {:flg; casein replace :} _test-nts >> NTS :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_allow_valid_flags() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; {:flg; partial replace casein :} _test-nts >> nts and more :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_allow_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:allow; {:flg; invalid_flag :} _test-nts >> nts and more :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
