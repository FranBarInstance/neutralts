#![doc = include_str!("../../doc/bif-filled.md")]

use crate::{bif::Bif, bif::BifError, constants::*, utils::*};

impl<'a> Bif<'a> {
    /*
        {:filled; varname >> ... :}

        *** It is only not filled if it has nothing "", if it is not defined
        or is null, the rest "false", "0" etc. is something.
    */
    pub(crate) fn parse_bif_filled(&mut self) -> Result<(), BifError> {
        if self.mod_filter {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(BifError {
                msg: "flags not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let mut varname = self.params.as_str();
        let mut schema = &self.shared.schema["data"];

        if varname.starts_with("local::") {
            schema = &self.shared.schema["__indir"][&self.inherit.indir]["data"];
            varname = varname.strip_prefix("local::").unwrap_or(varname);
        }

        if !is_empty_key(schema, varname) ^ self.mod_negate {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }

            self.out = self.code.to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_filled() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; __test-nts >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_obj_levels() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; __test-obj-nts->level1-obj->level2-obj->level2 >> {:;__test-obj-nts->level1-obj->level2-obj->level2:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_filled_evaluate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; __test-{:;__test-nts:} >> {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>nts</div>");
    }

    #[test]
    fn test_bif_filled_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+filled; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }

    #[test]
    fn test_bif_filled_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; __test-nts >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_true() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; true >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_false() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; false >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_text() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; text >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_zero() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; zero >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_one() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; one >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_spaces() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; spaces >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_empty() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; empty >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_null() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; null >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_emptyarr() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; emptyarr >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_array() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_true_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::true >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_false_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::false >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_text_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::text >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_zero_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::zero >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_one_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::one >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_spaces_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::spaces >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_empty_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::empty >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_null_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::null >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_emptyarr_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::emptyarr >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_array_local() {
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
            "<div>{:data; tests/local-data.json :}{:filled; local::array >> is filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_true_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; true >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_false_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; false >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_text_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; text >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_zero_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; zero >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_one_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; one >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_spaces_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; spaces >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_empty_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; empty >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_null_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; null >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_emptyarr_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; emptyarr >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_array_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_true_local_negate() {
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
            "<div>{:data; tests/local-data.json :}{:!filled; local::true >> is not filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_false_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::false >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_text_local_negate() {
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
            "<div>{:data; tests/local-data.json :}{:!filled; local::text >> is not filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_zero_local_negate() {
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
            "<div>{:data; tests/local-data.json :}{:!filled; local::zero >> is not filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_one_local_negate() {
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
            "<div>{:data; tests/local-data.json :}{:!filled; local::one >> is not filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_spaces_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::spaces >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_empty_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::empty >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_null_local_negate() {
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
            "<div>{:data; tests/local-data.json :}{:!filled; local::null >> is not filled :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_emptyarr_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::emptyarr >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_array_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_true() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->true >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_false() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->false >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_text() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->text >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_zero() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->zero >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_one() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->one >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_spaces() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->spaces >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_empty() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->empty >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_null() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; array->null >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_true_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->true >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_false_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->false >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_text_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->text >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_zero_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->zero >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_one_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->one >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_spaces_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->spaces >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_empty_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->empty >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_null_local() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:filled; local::array->null >> is filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_true_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->true >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_false_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->false >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_text_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->text >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_zero_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->zero >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_one_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->one >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_spaces_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->spaces >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_empty_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->empty >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_null_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!filled; array->null >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_true_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->true >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_false_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->false >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_text_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->text >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_zero_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->zero >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_one_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->one >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_spaces_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->spaces >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_filled_arr_empty_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->empty >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_arr_null_local_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:!filled; local::array->null >> is not filled :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>is not filled</div>");
    }

    #[test]
    fn test_bif_filled_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:filled; {:flg; invalid_flag :} __test-nts >> is filled :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
