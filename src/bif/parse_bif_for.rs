#![doc = include_str!("../../doc/bif-for.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
       {:for; varname 1 10 >>
           var is:{:;varname:}
       :}
    */
    pub(crate) fn parse_bif_for(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        self.params = self.params.replace("..", " ");
        let mut parts = self.params.split_whitespace();

        let var_name = match parts.next() {
            Some(value) => value.to_string(),
            None => {
                return Err(self.bif_error(BIF_ERROR_ARGUMENTS_NOT_FOUND));
            }
        };

        let from = match parts.next() {
            Some(value) => match value.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(self.bif_error(BIF_ERROR_ARGUMENT_NOT_NUMBER));
                }
            },
            None => {
                return Err(self.bif_error(BIF_ERROR_ARGS_FROM_TO_NOT_FOUND));
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
                return Err(self.bif_error(BIF_ERROR_ARGS_TO_NOT_FOUND));
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
#[path = "parse_bif_for_tests.rs"]
mod tests;
