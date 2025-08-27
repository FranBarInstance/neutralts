#![doc = include_str!("../../doc/bif-snippet.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, json, utils::*};
use std::collections::HashSet;

impl<'a> Bif<'a> {
    /*
        Play snippet:
        {:snippet; snippet-name :}

        Set snippet:
        {:snippet; snippet-name >>
            content to set
        :}
    */
    pub(crate) fn parse_bif_snippet(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.alias = "snippet".to_string();

        let is_set = self.extract_params_code(true);

        if !self.flags.is_empty() {
            let flags_allowed: HashSet<&str> = ["static"].into_iter().collect();

            for f in self.flags.split('|').filter(|s| !s.is_empty()) {
                if !flags_allowed.contains(f) {
                    return Err(self.bif_error(&format!("{} flag not allowed", f)));
                }
            }
        }

        if is_set {
            // Set snippets in snippet files and inside snippets
            if self.inherit.current_file.contains(SNIPPETS_FILES) || self.inherit.alias == "snippet"
            {
                if self.flags.contains("|static|") {
                    self.code = new_child_parse!(self, &self.code, self.mod_scope);
                } else {
                    // required regardless of mod_scope or static
                    self.inherit.create_block_schema(self.shared);
                }
                self.shared.schema["__indir"][&self.inherit.indir]["snippets"][&self.params] =
                    json!(&self.code);

                // The directory inside the snippet is that of the template that created it.
                self.shared.schema["__indir"][&self.inherit.indir]["snippets_set_dir"]
                    [&self.params] = json!(&self.inherit.current_dir);

                self.out = EMPTY_STRING;

                Ok(())
            } else {
                return Err(self.bif_error("snippet cannot be set here"))
            }
        } else {
            // parse snippet name if need
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
            }
            let snip_name = self.code.clone();

            self.code = get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["snippets"],
                &self.code,
            );

            if self.code.contains(BIF_OPEN) {
                // The directory inside the snippet is that of the template that created it.
                let set_dir = get_from_key(
                    &self.shared.schema["__indir"][&self.inherit.indir]["snippets_set_dir"],
                    &snip_name,
                );

                if !set_dir.is_empty() {
                    self.inherit.current_dir = set_dir;
                }

                // auto mod_scope in snippets for snippets inside snippets
                self.code = new_child_parse!(self, &self.code, self.code.contains("{:snip"));
            }

            self.out = self.code.to_string();

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_snippet() {
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
            .set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_snip() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:snip; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_snippet_evalueation() {
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
        "<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet-{:;__test-nts:} :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet nts</div></div>");
    }

    #[test]
    fn test_bif_snip_evalueation() {
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
            "<div>{:include; tests/snippets.ntpl :}{:snip; test-snippet-{:;__test-nts:} :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet nts</div></div>");
    }

    #[test]
    fn test_bif_snippet_nested() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet-nested :}{:snippet; test-snippet-nested-next :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet nested</div></div>");
    }

    #[test]
    fn test_bif_snip_nested() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/snippets.ntpl :}{:snip; test-snippet-nested :}{:snip; test-snippet-nested-next :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet nested</div></div>");
    }

    #[test]
    fn test_bif_snippet_nested_set_in_code() {
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
            "<div>{:include; tests/snippets.ntpl :}{:snippet; test-snippet-code :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet code</div></div>");
    }

    #[test]
    fn test_bif_snippet_no_scope() {
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
            "<div>{:* error, unnecessary scope, it is auto *:}{:+snippet; test-snippet :}</div>",
        );
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_snippet_no_negate() {
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
            .set_src_str("<div>{:include; tests/snippets.ntpl :}{:!snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
