#![doc = include_str!("../../doc/bif-sum.md")]

use crate::{bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
       {:sum; /a/b/ :}
    */
    pub(crate) fn parse_bif_sum(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.params = self.src.clone();

        if self.params.contains("{:flg;") {
            return Err(BifError {
                msg: "flags not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let args = self.extract_args();

        let param1_str = args.get(1).cloned().ok_or_else(|| BifError {
            msg: "arguments not found".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        let param2_str = args.get(2).cloned().ok_or_else(|| BifError {
            msg: "arguments not found".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        let param1: f64 = param1_str.parse().map_err(|_| BifError {
            msg: "invalid argument 1".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        let param2: f64 = param2_str.parse().map_err(|_| BifError {
            msg: "invalid argument 2".to_string(),
            name: self.alias.clone(),
            src: self.raw.to_string(),
        })?;

        self.out = (param1 + param2).to_string();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_sum() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /1/2/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>3</div>");
    }

    #[test]
    fn test_bif_sum_decimals() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /1.5/2.1/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>3.6</div>");
    }

    #[test]
    fn test_bif_sum_eval() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /{:;one:}/{:;one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>2</div>");
    }

    #[test]
    fn test_bif_sum_nested() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /{:;one:}/{:sum;|{:;one:}|{:;one:}|:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>3</div>");
    }

    #[test]
    fn test_bif_sum_subtract() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /{:;one:}/-{:;one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0</div>");
    }

    #[test]
    fn test_bif_sum_subtract_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:sum; /-{:;one:}/{:;one:}/ :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0</div>");
    }
}
