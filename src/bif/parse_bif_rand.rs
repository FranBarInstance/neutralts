#![doc = include_str!("../../doc/bif-rand.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*};
use rand::Rng;

impl<'a> Bif<'a> {
    /*
        {:rand;  :}
        {:rand; 1..100 :}
    */
    pub(crate) fn parse_bif_rand(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
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
                        return Err(self.bif_error(BIF_ERROR_ARGUMENT_NOT_NUMBER));
                    }
                },
                None => {
                    return Err(self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND));
                }
            };

            let to = match parts.next() {
                Some(value) => match value.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        return Err(self.bif_error(BIF_ERROR_ARGUMENT_NOT_NUMBER));
                    }
                },
                None => {
                    return Err(self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND));
                }
            };

            if from > to {
                return Err(self.bif_error(BIF_ERROR_FROM_GREATER_THAN_TO));
            }

            self.out = rng.random_range(from..=to).to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "parse_bif_rand_tests.rs"]
mod tests;
