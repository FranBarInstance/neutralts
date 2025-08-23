#![doc = include_str!("../../doc/bif-date.md")]

use crate::{bif::Bif, bif::BifError, constants::*};
use chrono::Utc;

impl<'a> Bif<'a> {
    /*
        {:date;  :} timestamp
        {:date; %Y-%m-%d %H:%M:%S  :} UTC
    */
    pub(crate) fn parse_bif_date(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);

        let now = Utc::now();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.src.trim().to_string();
        }

        if self.code.is_empty() {
            self.out = now.timestamp().to_string();
        } else {
            self.out = now.format(&self.src).to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_date_timestamp() {
        use std::time::SystemTime;
        fn is_timestamp(value: u64) -> bool {
            SystemTime::UNIX_EPOCH
                .checked_add(std::time::Duration::from_secs(value))
                .is_some()
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
        template.set_src_str("{:date; :}");
        let result = template.render().parse::<u64>().unwrap();

        assert!(!template.has_error());
        assert!(is_timestamp(result));
    }

    #[test]
    fn test_bif_date() {
        use chrono::{DateTime, Utc};
        use std::str::FromStr;
        pub fn is_valid_rfc3339(value: &str) -> bool {
            DateTime::<Utc>::from_str(value).is_ok()
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
        template.set_src_str("{:date; %Y-%m-%d %H:%M:%S :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(!is_valid_rfc3339(&result));
    }
}
