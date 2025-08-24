#![doc = include_str!("../../doc/bif-for.md")]

use crate::{bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
       {:for; varname 1 10 >>
           var is:{:;varname:}
       :}
    */
    pub(crate) fn parse_bif_for(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        self.extract_params_code(true);
        self.params = self.params.replace("..", " ");
        let mut parts = self.params.split_whitespace();

        let var_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(BifError {
                    msg: "arguments not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let from = match parts.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(BifError {
                        msg: "argument is not a number".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            },
            None => {
                return Err(BifError {
                    msg: "arguments 'from' and 'to' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let to = match parts.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(BifError {
                        msg: "argument is not a number".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            },
            None => {
                return Err(BifError {
                    msg: "arguments 'to' not found".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                })
            }
        };

        let range = if from > to {
            (to..=from).rev().collect::<Vec<i32>>()
        } else {
            (from..=to).collect::<Vec<i32>>()
        };

        let restore_var = self.get_data(&var_name);
        for i in range {
            self.set_data(&var_name, &i.to_string());
            self.out += &new_child_parse!(self, &self.code, self.mod_scope);
        }
        self.set_data(&var_name, &restore_var);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_for() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; n 0 9 >> {:;n:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>0123456789</div>");
    }

    #[test]
    fn test_bif_for_rev() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; n 9 0 >> {:;n:} :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>9876543210</div>");
    }

    #[test]
    fn test_bif_for_params_fails_1() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; n a b >> {:;n:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_for_params_fails_2() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; n a >> {:;n:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_for_params_fails_3() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; n >> {:;n:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_for_params_fails_4() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; >> {:;n:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_for_no_negate() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:!for; n 0 9 >> {:;n:} :}</div>");
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_for_no_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:for; n 1..1 >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div></div>");
    }

    #[test]
    fn test_bif_for_scope() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str("<div>{:+for; n 1..1 >> {:include; tests/snippets.ntpl :} :}{:snippet; test-snippet :}</div>");
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div><div>test snippet</div></div>");
    }
}
