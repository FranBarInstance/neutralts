#![doc = include_str!("../../doc/bif-cache.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, utils::*};
use md5::Digest;
use sha2::Sha256;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl<'a> Bif<'a> {
    /*
        {:cache; /expires/id/only_custom_id/ >> ... :} {:* expires in seconds *:}
        {:cache; /expires/id/ >> ... :}
        {:cache; /expires/ >> ... :}
        {:!cache; ... :}
    */
    pub(crate) fn parse_bif_cache(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        self.extract_params_code(false);

        if self.params.contains("{:flg;") {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.mod_negate {
            if self.inherit.in_cache {
                self.out = self.raw.to_string();
            } else {
                // If it is not in a cache block, it is now resolved.
                self.out = new_child_parse!(self, &self.code, self.mod_scope);
            }
            return Ok(());
        }

        let restore_in_cache = self.inherit.in_cache;
        let context = &self.shared.schema["data"]["CONTEXT"];
        let has_post = !is_empty_key(context, "POST");
        let has_get = !is_empty_key(context, "GET");
        let has_cookies = !is_empty_key(context, "COOKIES");

        if self.shared.cache_disable
            || (has_post && !self.shared.cache_on_post)
            || (has_get && !self.shared.cache_on_get)
            || (has_cookies && !self.shared.cache_on_cookies)
        {
            if self.code.contains(BIF_OPEN) {
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
            }
            self.out = self.code.clone();
            return Ok(());
        }

        self.inherit.in_cache = true;
        let args = self.extract_args();
        self.inherit.in_cache = restore_in_cache;

        // require expires
        let expires = args
            .get(1)
            .cloned()
            .ok_or_else(|| self.bif_error("arguments 'expires' not found"))?;

        // optional id
        let mut id = args.get(2).cloned().unwrap_or("".to_string());

        // optional only_custom_id
        let only_custom_id: bool = match args.get(3) {
            Some(value) => !matches!(value.as_str(), "false" | "0" | ""),
            None => false,
        };

        if !only_custom_id {
            id.push_str(&self.shared.lang);
            id.push_str(&expires);
            if has_post {
                id.push_str(
                    &serde_json::to_string(&self.shared.schema["data"]["CONTEXT"]["POST"]).unwrap(),
                );
            }
            if has_get {
                id.push_str(
                    &serde_json::to_string(&self.shared.schema["data"]["CONTEXT"]["GET"]).unwrap(),
                );
            }
            if has_cookies {
                id.push_str(
                    &serde_json::to_string(&self.shared.schema["data"]["CONTEXT"]["COOKIES"])
                        .unwrap(),
                );
            }
            id.push_str(&self.get_data("CONTEXT->HOST"));
            id.push_str(&self.get_data("CONTEXT->ROUTE"));
            id.push_str(&self.code);
        }

        let mut hasher = Sha256::new();
        hasher.update(id.clone());
        let cache_id = format!("{:x}", hasher.finalize());
        let cache_dir = self.get_cache_dir(&cache_id);
        let file = format!("{}/{}-{}", cache_dir, &cache_id, expires);
        let file_path = Path::new(&file);

        if file_path.exists()
            && !self.cache_file_expires(file_path, expires.parse::<u64>().unwrap_or(0))
        {
            if let Ok(content) = fs::read_to_string(file_path) {
                self.out = content;
            } else {
                // The output is created even if there is an error
                if self.code.contains(BIF_OPEN) {
                    self.inherit.in_cache = true;
                    self.out = new_child_parse!(self, &self.code, self.mod_scope);
                    self.inherit.in_cache = restore_in_cache;
                }
                return Err(
                    self.bif_error(&format!("Failed to read cache {}", file_path.display()))
                );
            }
        } else {
            if self.code.contains(BIF_OPEN) {
                self.inherit.in_cache = true;
                self.code = new_child_parse!(self, &self.code, self.mod_scope);
                self.inherit.in_cache = restore_in_cache;
            }

            // The output is created even if there is an error
            self.out = self.code.clone();

            // Create cache dir
            self.set_cache_dir(&cache_dir)?;

            // Write in cache
            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&self.code.as_bytes()) {
                        return Err(self.bif_error(&format!(
                            "Failed to write to cache {}: {}",
                            file_path.display(),
                            e.to_string()
                        )));
                    }
                }
                Err(e) => {
                    return Err(self.bif_error(&format!(
                        "Failed to create file {}: {}",
                        file_path.display(),
                        e.to_string()
                    )))
                }
            }
        }

        Ok(())
    }

    pub(crate) fn cache_file_expires(&self, file_path: &Path, expires: u64) -> bool {
        let now: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let metadata = match fs::metadata(file_path) {
            Ok(meta) => meta,
            Err(_) => return true,
        };

        let modified_time = match metadata.modified() {
            Ok(time) => time,
            Err(_) => return true,
        };

        let duration_since_epoch = match modified_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(_) => return true,
        };

        let file_modified_time = duration_since_epoch.as_secs();
        let expiration_time = file_modified_time + expires;

        if now > expiration_time {
            return true;
        }

        false
    }

    pub(crate) fn set_cache_dir(&self, cache_dir: &str) -> Result<(), BifError> {
        let cache_dir_levels = Path::new(&cache_dir);

        match fs::create_dir_all(cache_dir_levels) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(self.bif_error(&format!(
                    "Failed to create cache directory {}: {}",
                    cache_dir,
                    e.to_string()
                )))
            }
        }
    }

    pub(crate) fn get_cache_dir(&self, file: &str) -> String {
        let mut cache_dir = self.shared.cache_dir.clone();

        if !self.shared.cache_prefix.is_empty() {
            cache_dir.push_str("/");
            cache_dir.push_str(&self.shared.cache_prefix);
        }

        cache_dir.push_str("/");
        cache_dir.push_str(&file[0..3]);

        cache_dir.to_string()
    }
}

#[cfg(test)]
#[path = "parse_bif_cache_tests.rs"]
mod tests;
