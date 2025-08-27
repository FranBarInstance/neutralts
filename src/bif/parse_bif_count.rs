#![doc = include_str!("../../doc/bif-count.md")]

/*
    .------------.
    | DEPRECATED |
    '------------'
*/

use crate::{
    constants::*,
    bif::Bif,
    bif::BifError,
    bif::constants::*,
};

impl<'a> Bif<'a> {
    /*
        This bif is poorly designed, it also sets the values globally
        which can cause variables to be overwritten.

        {:count; name >> 0 :}
        {:count; name :}
    */
    pub(crate) fn parse_bif_count(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        let is_set = self.extract_params_code(true);

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        if is_set {
            let count_name = self.params.clone();
            let count_value = match self.code.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(self.bif_error(BIF_ERROR_ARGUMENT_NOT_NUMBER));
                }
            };

            self.set_data(&count_name, &count_value.to_string());
            self.out = EMPTY_STRING;
        } else {
            let count_name = self.code.clone();
            let count_value = match self.get_data(&count_name).parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(self.bif_error(BIF_ERROR_ARGUMENT_NOT_NUMBER));
                }
            };
            let new_value = count_value + 1;

            self.set_data(&count_name, &new_value.to_string());
            self.out = count_value.to_string();
        }

        Err(self.bif_error(BIF_ERROR_BIF_DEPRECATED))
    }
}
