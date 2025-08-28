use crate::{bif::constants::*, bif::Bif, bif::BifError};

impl<'a> Bif<'a> {
    /*
        unknown bif
    */
    pub(crate) fn parse_bif_unknown(&mut self) -> Result<(), BifError> {
        self.alias = "unknown".to_string();

        Err(self.bif_error(BIF_ERROR_UNKNOWN_BIF))
    }
}

#[cfg(test)]
#[path = "parse_bif_unknown_tests.rs"]
mod tests;
