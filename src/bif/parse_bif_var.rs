#![doc = include_str!("../../doc/bif-var.md")]

use crate::{
    constants::*,
    utils::*,
    bif::Bif,
    bif::BifError,
};

impl<'a> Bif<'a> {
    /*
        {:;varname:}
        {:;:}
    */
    pub(crate) fn parse_bif_var(&mut self) -> Result<(), BifError> {
        if self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        // Unprintable: {:;:} / {:; :}
        if self.src.is_empty() {
            // "bif.alias" is used and not "bif.name" because in "var" or "unprintable"
            // its name is an empty string.
            self.alias = "unprintable".to_string();

            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        // Var: {:;varname:}
        self.alias = "var".to_string();
        let var_name;

        // For security requires {:allow; in some cases.
        if self.src.contains(BIF_OPEN) {
            if !self.contains_allow(&self.src) {
                self.out = EMPTY_STRING;

                return Err(BifError {
                    msg: "insecure varname".to_string(),
                    name: self.alias.clone(),
                    src: self.src.clone(),
                });
            }

            var_name = new_child_parse!(self, &self.src, self.mod_scope);
        } else {
            var_name = self.src.clone();
        }

        self.out = self.get_data(&var_name).to_string();

        if (self.mod_filter || self.shared.filter_all) && !self.mod_negate {
            if !var_name.starts_with("CONTEXT->") {
                // unescape_chars for prevent double encoding
                self.out = unescape_chars(&self.get_data(&var_name), true).to_string();
                self.out = escape_chars(&self.out, true).to_string();
            }
        } else {
            if self.shared.filter_bifs {
                // Avoid reevaluation in cache
                self.out = self.out.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
                self.out = self.out.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
            }
        }

        if self.mod_negate && !self.shared.filter_bifs && var_name.starts_with("CONTEXT->") {
            self.out = unescape_chars(&self.out, true).to_string();
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_var() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-nts:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_var_arr() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__test-obj-nts->level1-obj->level2-obj->level3-arr->0:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>one</div>");
    }

    #[test]
    fn test_bif_var_dynamic_evaluation() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__hello-{:;__test-nts:}:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Hello nts</div>");
    }

    #[test]
    fn test_bif_var_error_dynamic_evaluation() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;{:;__ref-hello-nts:}:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_var_undefined() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;__dfhs76tfwq65dhtw563hjknv__:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_var_upline() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("|  \n  {:^;__test-nts:}<div></div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "|nts<div></div>");
    }

    #[test]
    fn test_bif_var_filter() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:&;escape:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&lt;&gt;&amp;&quot;&#x27;&#x2F;&#123;&#125;</div>");
    }

    #[test]
    fn test_bif_var_filter_double() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:&;double_escape:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&lt;&gt;&amp;&quot;&#x27;&#x2F;&#123;&#125;</div>");
    }

    #[test]
    fn test_bif_var_filter_context() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:;CONTEXT->GET->escape:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&lt;&gt;&amp;&quot;&#x27;&#x2F;&#123;&#125;</div>");
    }

    #[test]
    fn test_bif_var_filter_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:&!;escape:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><>&\"'/{}</div>");
    }

    #[test]
    fn test_bif_var_filter_all() {
        let schema = r#"
        {
            "config": {
                "filter_all": true
            }
        }
        "#
        .trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:;escape:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&lt;&gt;&amp;&quot;&#x27;&#x2F;&#123;&#125;</div>");
    }

    #[test]
    fn test_bif_var_filter_all_negate() {
        let schema = r#"
        {
            "config": {
                "filter_all": true
            }
        }
        "#
        .trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str("<div>{:!;escape:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><>&\"'/{}</div>");
    }

    #[test]
    fn test_bif_var_error_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+;__test-nts:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
