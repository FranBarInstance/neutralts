#![doc = include_str!("../../doc/bif-include.md")]

use crate::{bif::Bif, bif::BifError, constants::*, utils::*};
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:include; file-path :}
        {:include; {:flg; require safe noparse :} >> file-path :}
    */
    pub(crate) fn parse_bif_include(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        self.file_path = self.code.clone();

        // For security requires {:allow;
        if self.file_path.contains(BIF_OPEN) {
            if !self.contains_allow(&self.file_path) {
                return Err(BifError {
                    msg: "insecure file name".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }
            self.file_path = new_child_parse!(self, &self.code, false);
        }

        if self.file_path.starts_with("#") {
            self.file_path.remove(0);
            self.file_path = format!("{}{}", self.inherit.current_dir, self.file_path);
        }

        let path = Path::new(&self.file_path);
        if !path.exists() {
            if self.flags.contains("|require|") {
                return Err(BifError {
                    msg: "file not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            } else {
                return Ok(());
            }
        }

        if let Some(parent) = path.parent() {
            self.dir = parent.display().to_string();
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if self.mod_negate && self.inherit.include_files.contains(&canonical_path) {
            self.out = EMPTY_STRING;

            return Ok(());
        }

        if self.flags.contains("|safe|") {
            self.code = fs::read_to_string(&self.file_path).unwrap_or("".to_string());
            self.code = escape_chars(&unescape_chars(&self.code, false), false).to_string();
            self.code = self.code.replace(BIF_OPEN, BIF_SANITIZE_OPEN);
            self.code = self.code.replace(BIF_CLOSE, BIF_SANITIZE_CLOSE);
            self.out = self.code.clone();

            return Ok(());
        }

        if self.flags.contains("|noparse|") {
            self.code = fs::read_to_string(&self.file_path).unwrap_or("".to_string());
            self.out = self.code.clone();

            return Ok(());
        }

        self.inherit.include_files.push(canonical_path);

        let mut file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());
        if self.shared.comments.contains("remove") {
            file_raw = remove_comments(&file_raw);
        }

        self.out = new_child_parse!(self, &file_raw, true);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_include() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/include-snippets.ntpl :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_include_flag_require() {
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
            "<div>{:include; {:flg; require :} >> tests/include-snippets.ntpl :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_include_flag_require_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; {:flg; require :} >> tests/not-found.ntpl :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_include_not_found() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/not-found.ntpl :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_include_allow() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; {:allow; any >> {:;__test-nts:} :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_include_allow_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_include_without_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/include-snippets.ntpl :}{:include; tests/include-snippets.ntpl :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>OkOk</div>");
    }

    #[test]
    fn test_bif_include_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; tests/include-snippets.ntpl :}{:!include; tests/include-snippets.ntpl :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_include_text_files() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:include; {:flg; safe :} >> tests/include.txt :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>Lorem Ipsum &lt;div&gt;&#123;:code; :&#125;&lt;&#x2F;div&gt;</div>"
        );
    }

    #[test]
    fn test_bif_include_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:* error, unnecessary scope, it is auto *:}{:+include; tests/include-snippets.ntpl :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
