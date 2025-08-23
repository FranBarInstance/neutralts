#![doc = include_str!("../../doc/bif-rand.md")]

use crate::{bif::Bif, bif::BifError, constants::*};
use rand::Rng;

impl<'a> Bif<'a> {
    /*
        {:rand;  :}
        {:rand; 1..100 :}
    */
    pub(crate) fn parse_bif_rand(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(BifError {
                msg: "modifier not allowed".to_string(),
                name: self.alias.clone(),
                src: self.raw.to_string(),
            });
        }

        let mut rng = rand::rng();
        self.code = self.src.trim().to_string();

        if self.src.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        } else {
            self.code = self.src.trim().to_string();
        }

        if self.code.is_empty() {
            self.out = rng.random_range(100000000..=999999999).to_string();
        } else {
            // TODO comprobar rangos
            self.code = self.code.replace("..", " ");
            let mut parts = self.code.split_whitespace();

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
                        msg: "arguments not found".to_string(),
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
                        msg: "arguments not found".to_string(),
                        name: self.alias.clone(),
                        src: self.raw.to_string(),
                    })
                }
            };

            if from > to {
                return Err(BifError {
                    msg: "from > to".to_string(),
                    name: self.alias.clone(),
                    src: self.raw.to_string(),
                });
            }

            self.out = rng.random_range(from..=to).to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_rand() {
        pub fn number(s: &str, x: usize) -> bool {
            s.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .len()
                == x
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
        template.set_src_str("{:rand; :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(number(&result, 9));
    }

    #[test]
    fn test_bif_rand_10_99() {
        pub fn number(s: &str, x: usize) -> bool {
            s.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .len()
                == x
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
        template.set_src_str("{:rand; 10..99 :}");
        let result = template.render();
        assert!(!template.has_error());
        assert!(number(&result, 2));
    }
}
