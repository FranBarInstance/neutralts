#![doc = include_str!("../../doc/bif-data.md")]

use std::collections::HashSet;
use crate::{bif::Bif, bif::BifError, constants::*, utils::*, Value};
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:data; file-path :} {:* local data *}
    */
    pub(crate) fn parse_bif_data(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            let flags_allowed: HashSet<&str> = [
                "inline",
                "require"
            ].into_iter().collect();

            for f in self.flags.split('|').filter(|s| !s.is_empty()) {
                if !flags_allowed.contains(f) {
                    return Err(BifError {
                        msg: format!("{} flag not allowed", f),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    });
                }
            }
        }

        if self.flags.contains("|inline|") {
            let data: Value = match serde_json::from_str(&self.code) {
                Ok(value) => value,
                Err(_) => {
                    return Err(BifError {
                        msg: "not a valid JSON file".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            };

            let indir = &self.inherit.create_block_schema(self.shared);

            // Merge new locale data in curren local data.
            merge_schema(
                &mut self.shared.schema["__indir"][indir]["data"],
                &data["data"],
            );

            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

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

        if let Some(stripped) = self.file_path.strip_prefix('#') {
            self.file_path = format!("{}{}", self.inherit.current_dir, stripped);
        }

        let path = Path::new(&self.file_path);
        if !Path::new(path).exists() {
            if self.flags.contains("|require|") {
                return Err(BifError {
                    msg: "file not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            } else {
                self.out = EMPTY_STRING;

                return Ok(());
            }
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if self.mod_negate && self.inherit.data_files.contains(&canonical_path) {
            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        self.inherit.data_files.push(canonical_path);
        let file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());

        let data: Value = match serde_json::from_str(&file_raw) {
            Ok(value) => value,
            Err(_) => {
                return Err(BifError {
                    msg: "not a valid JSON file".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let indir = &self.inherit.create_block_schema(self.shared);

        // Merge new locale data in curren local data.
        merge_schema(
            &mut self.shared.schema["__indir"][indir]["data"],
            &data["data"],
        );

        self.out = UNPRINTABLE.to_string();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_data() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; tests/local-data.json :}{:;local::hello:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>local hello</div>");
    }

    #[test]
    fn test_bif_data_no_evaluation() {
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
            "<div>{:data; tests/{:;__test-local:}-data.json :}{:;local::hello-nts:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>{:;__test-nts:}</div>");
    }

    #[test]
    fn test_bif_data_flag_require() {
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
            "<div>{:data; {:flg; require :} tests/local-data.json :}{:;local::hello:}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>local hello</div>");
    }

    #[test]
    fn test_bif_data_flag_require_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; {:flg; require :} >> tests/not-found.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_data_allow() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; {:allow; any >> {:;__test-nts:} :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_data_allow_fails() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_data_inline() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; {:flg; inline :} >> { \"data\": { \"hello\": \"local hello\" } } :}{:;local::hello:}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>local hello</div>");
    }

    #[test]
    fn test_bif_data_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:data; {:flg; invalid_flag :} >> tests/local-data.json :}{:;local::hello:}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
