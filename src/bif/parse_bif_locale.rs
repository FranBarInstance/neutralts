#![doc = include_str!("../../doc/bif-locale.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*, Value};
use std::fs;
use std::path::Path;

impl<'a> Bif<'a> {
    /*
        {:locale; file-path :}
    */
    pub(crate) fn parse_bif_locale(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            if !self.flags.contains("|require|")
                && !self.flags.contains("|inline|")
                && !self.flags.contains("|noparse|")
            {
                return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
            }
        }

        if self.flags.contains("|inline|") {
            // Parse possible bifs included in json
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, false);
            }

            let locale: Value = match serde_json::from_str(&self.code) {
                Ok(value) => value,
                Err(_) => {
                    return Err(self.bif_error(BIF_ERROR_NOT_VALID_JSON));
                }
            };

            let indir = &self.inherit.create_block_schema(self.shared);

            // Merge new locale data in curren locale.
            merge_schema(&mut self.shared.schema["__indir"][indir]["locale"], &locale);

            self.out = EMPTY_STRING;

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
                return Ok(());
            }
        }

        let canonical_path = fs::canonicalize(path)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if self.mod_negate && self.inherit.locale_files.contains(&canonical_path) {
            self.out = UNPRINTABLE.to_string();

            return Ok(());
        }

        self.inherit.locale_files.push(canonical_path);
        let mut file_raw = fs::read_to_string(&self.file_path).unwrap_or("".to_string());

        if !self.flags.contains("|noparse|") {
            // Parse possible bifs included in json
            if file_raw.contains(BIF_OPEN) {
                file_raw = new_child_parse!(self, &file_raw, false);
            }
        }

        let locale: Value = match serde_json::from_str(&file_raw) {
            Ok(value) => value,
            Err(_) => {
                return Err(self.bif_error(BIF_ERROR_NOT_VALID_JSON));
            }
        };

        let indir = &self.inherit.create_block_schema(self.shared);

        // Merge new locale data in curren locale.
        merge_schema(&mut self.shared.schema["__indir"][indir]["locale"], &locale);

        self.out = UNPRINTABLE.to_string();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_locale() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str("<div>{:locale; tests/locale.es.json :}{:trans; test-locale :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_locale_evaluation() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str(
            "<div>{:locale; tests/locale.{:lang;:}.json :}{:trans; Test {:;__test-nts:} :}</div>",
        );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok nts</div>");
    }

    #[test]
    fn test_bif_locale_flag_inline() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
        }
    }
    "#
        .trim();
        let source = r#"
        {:locale; {:flg; inline :} >>
            {
                "trans": {
                    "{:lang;:}": {
                        "test-locale": "inline"
                    }
                }
            }
        :}
        <div>{:trans; test-locale :}</div>
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
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>inline</div>");
    }

    #[test]
    fn test_bif_locale_flag_require() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str(
        "<div>{:locale; {:flg; require :} >> tests/locale.es.json :}{:trans; test-locale :}</div>",
    );
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_locale_flag_require_fails() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template
            .set_src_str("<div>{:locale; {:flg; require :} >> tests/not-found.es.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_not_found() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template
            .set_src_str("<div>{:locale; tests/not-found.es.json :}{:trans; test-locale :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>test-locale</div>");
    }

    #[test]
    fn test_bif_locale_allow() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str("<div>{:locale; {:allow; any >> {:;__test-nts:} :} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_allow_fails() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str("<div>{:locale; {:;__test-nts:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_negate() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str("<div>{:!locale; tests/locale.es.json :}{:!locale; tests/locale.es.json :}{:trans; test-locale :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>Ok</div>");
    }

    #[test]
    fn test_bif_locale_scope() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template.set_src_str("<div>{:+locale; tests/locale.es.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_locale_invalid_flag() {
        let schema = r#"
    {
        "inherit": {
            "locale": {
                "current": "es"
            }
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
        template
            .set_src_str("<div>{:locale; {:flg; invalid_flag :} >> tests/locale.es.json :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }
}
