#![doc = include_str!("../../doc/bif-for.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, utils::extract_blocks};

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

        let blocks = match extract_blocks(&self.code) {
            Ok(b) => b,
            Err(p) => return Err(self.bif_error(&format!("Unmatched block at position {}", p))),
        };

        let restore_var = self.get_data(&var_name);
        if from > to {
            for i in (to..=from).rev() {
                self.parse_bif_for_iter(&var_name, &i.to_string(), &blocks);
            }
        } else {
            for i in from..=to {
                self.parse_bif_for_iter(&var_name, &i.to_string(), &blocks);
            }
        };
        self.set_data(&var_name, &restore_var);

        Ok(())
    }

    fn parse_bif_for_iter(&mut self, var_name: &str, val: &str, blocks: &Vec<(usize, usize)>) {
        self.set_data(var_name, val);

        let mut child_inherit = self.inherit.clone();
        child_inherit.alias = self.alias.clone();
        if !self.file_path.is_empty() {
            child_inherit.current_file = self.file_path.clone();
        }
        if !self.dir.is_empty() {
            child_inherit.current_dir = self.dir.clone();
        }

        if self.mod_scope {
            self.inherit.create_block_schema(self.shared);
        }

        let mut block_parser = crate::block_parser::BlockParser::new(self.shared, &child_inherit);
        let code = block_parser.parse_with_blocks(&self.code, blocks, self.only);

        if self.mod_scope {
            block_parser.update_indir(&self.inherit.indir);
        }

        self.out += &code;
    }
}

#[cfg(test)]
#[path = "parse_bif_for_tests.rs"]
mod tests;
