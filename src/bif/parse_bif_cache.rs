#![doc = include_str!("../../doc/bif-cache.md")]

use crate::{bif::Bif, bif::BifError, bif::constants::*, constants::*, utils::*};
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
        let expires = args.get(1).cloned().ok_or_else(|| self.bif_error("arguments 'expires' not found"))?;


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
                return Err(self.bif_error(&format!("Failed to read cache {}", file_path.display())));
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
                        return Err(self.bif_error(
                            &format!("Failed to write to cache {}: {}",file_path.display(),e.to_string())
                        ));
                    }
                }
                Err(e) => {
                    return Err(self.bif_error(
                        &format!("Failed to create file {}: {}",file_path.display(),e.to_string())
                    ))
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
                return Err(self.bif_error(
                    &format!("Failed to create cache directory {}: {}", cache_dir, e.to_string())
                ))
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
mod tests {
    use crate::test_helpers::*;
    use crate::constants::*;
    use std::thread;
    use std::time;

    #[test]
    fn test_bif_cache() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = "<div>{:cache; /3/ >> {:;inject:} :}</div>";

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");

        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
    }

    #[test]
    fn test_bif_cache_mailfotmated_bif() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src =
            "<div>{:;mailfotmated:}{:cache; /3/ >> {:!;mailfotmated:} :}{:!;mailfotmated:}</div>";

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>&#123;::&#125;&#123;::&#125;&#123;::&#125;</div>"
        );

        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(
            result,
            "<div>&#123;::&#125;&#123;::&#125;&#123;::&#125;</div>"
        );
    }

    #[test]
    fn test_bif_inject_cache() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        // It should be more than 2 seconds to make sure it works, and no more than 3
        // seconds to make sure that if the test is re-run it updates.
        // result: "&#123;:exit; 403 :&#125; XX &#123;:exit; 403 :&#125; XX"
        let src =
            "{:cache; /3/ >> {:;inject:} {:date; %S :} {:!cache; {:;inject:} {:date; %S :} :} :}";

        // write cache
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.set_src_str(src);
        let result_write = template.render();
        assert!(!template.has_error());

        // the part up to where the seconds appear in cache
        assert_eq!(&result_write[0..25], "&#123;:exit; 403 :&#125; ");

        // the part up to where the seconds appear in !cache
        assert_eq!(&result_write[28..53], "&#123;:exit; 403 :&#125; ");

        // we give 1 second for “date” to show a different result in !cache
        thread::sleep(time::Duration::from_secs(1));

        // read cache
        template.set_src_str(src);
        let result_read = template.render();
        assert!(!template.has_error());

        // the first part in cache must be the same
        assert_eq!(&result_write[0..53], &result_read[0..53]);

        // the second part in !cache must be different
        assert!(&result_write != &result_read[0..53]);
    }

    #[test]
    fn test_bif_cache_complete() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = r#"
        {:^include; {:flg; require :} >> tests/snippets.ntpl :}
        {:^locale; tests/locale.{:lang;:}.json :}
        {:^;:}<div1></div1>
        {:^;:}<div2></div2>
        {:^;:}<div3></div3>
        {:^;:}::--::{:^date; %S :}::--::
        {:^;:}{:sum; /{:;one:}/{:;one:}/ :}
        {:^;:}{:fetch; '/url' >> loading... :}
        {:^;:}{:join; /__test-arr-nts/|/ :}
        {:^;:}{:;__hello-nts:}
        {:^;:}{:allow; _test-nts >> {:;__hello-nts:} :}
        {:^;:}{:!allow; _test-nts >> {:;__hello-nts:} :}
        {:^;:}{:array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
        {:^;:}{:!array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
        {:^;:}{:bool; true >> true :}
        {:^;:}{:!bool; true >> true :}
        {:^;:}{:coalesce; {:;empty:}{:;__hello-nts:} :}
        {:^;:}{:code; {:param; {:;__hello-nts:} >> {:;__hello-nts:} :} {:coalesce; {:;empty:}{:param; {:;__hello-nts:} :} :} :}
        {:^;:}{:contains; /haystack/st/ >> contains :}
        {:^;:}{:defined; __test-nts >> is defined :}
        {:^;:}{:!defined; __test-nts >> is defined :}
        {:^;:}{:code;  :}{:else; else :}
        {:^;:}{:eval; {:;__test-nts:} >> {:;__eval__:} :}
        {:^;:}{:filled; __test-nts >> is filled :}
        {:^;:}{:!filled; __test-nts >> is filled :}
        {:^;:}{:for; n 0 9 >> {:;n:} :}
        {:^;:}{:hash; {:;__test-nts:} :}
        {:^;:}{:lang; :}
        {:^;:}{:moveto; <div1 >> 1{:;__test-nts:} :}
        {:^;:}{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}
        {:^;:}{:rand; 1..1 :}
        {:^;:}{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}
        {:^;:}{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}
        {:^;:}{:trans; {:trans; Hello nts :} :}
        {:^cache; /3/ >>
            {:^;:}::--::{:^date; %S :}::--::
            {:^;:}{:sum; /{:;one:}/{:;one:}/ :}
            {:^;:}{:fetch; '/url' >> loading... :}
            {:^;:}{:join; /__test-arr-nts/|/ :}
            {:^;:}{:;__hello-nts:}
            {:^;:}{:allow; _test-nts >> {:;__hello-nts:} :}
            {:^;:}{:!allow; _test-nts >> {:;__hello-nts:} :}
            {:^;:}{:array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
            {:^;:}{:!array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
            {:^;:}{:bool; true >> true :}
            {:^;:}{:!bool; true >> true :}
            {:^;:}{:coalesce; {:;empty:}{:;__hello-nts:} :}
            {:^;:}{:code; {:param; {:;__hello-nts:} >> {:;__hello-nts:} :} {:coalesce; {:;empty:}{:param; {:;__hello-nts:} :} :} :}
            {:^;:}{:contains; /haystack/st/ >> contains :}
            {:^;:}{:defined; __test-nts >> is defined :}
            {:^;:}{:!defined; __test-nts >> is defined :}
            {:^;:}{:code;  :}{:else; else :}
            {:^;:}{:eval; {:;__test-nts:} >> {:;__eval__:} :}
            {:^;:}{:filled; __test-nts >> is filled :}
            {:^;:}{:!filled; __test-nts >> is filled :}
            {:^;:}{:for; n 0 9 >> {:;n:} :}
            {:^;:}{:hash; {:;__test-nts:} :}
            {:^;:}{:lang; :}
            {:^;:}{:moveto; <div2 >> 2{:;__test-nts:} :}
            {:^;:}{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}
            {:^;:}{:rand; 1..1 :}
            {:^;:}{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}
            {:^;:}{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}
            {:^;:}{:trans; {:trans; Hello nts :} :}
            {:!cache;
                {:^;:}::--::{:^date; %S :}::--::
                {:^;:}{:sum; /{:;one:}/{:;one:}/ :}
                {:^;:}{:fetch; '/url' >> loading... :}
                {:^;:}{:join; /__test-arr-nts/|/ :}
                {:^;:}{:;__hello-nts:}
                {:^;:}{:allow; _test-nts >> {:;__hello-nts:} :}
                {:^;:}{:!allow; _test-nts >> {:;__hello-nts:} :}
                {:^;:}{:array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
                {:^;:}{:!array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
                {:^;:}{:bool; true >> true :}
                {:^;:}{:!bool; true >> true :}
                {:^;:}{:coalesce; {:;empty:}{:;__hello-nts:} :}
                {:^;:}{:code; {:param; {:;__hello-nts:} >> {:;__hello-nts:} :} {:coalesce; {:;empty:}{:param; {:;__hello-nts:} :} :} :}
                {:^;:}{:contains; /haystack/st/ >> contains :}
                {:^;:}{:defined; __test-nts >> is defined :}
                {:^;:}{:!defined; __test-nts >> is defined :}
                {:^;:}{:code;  :}{:else; else :}
                {:^;:}{:eval; {:;__test-nts:} >> {:;__eval__:} :}
                {:^;:}{:filled; __test-nts >> is filled :}
                {:^;:}{:!filled; __test-nts >> is filled :}
                {:^;:}{:for; n 0 9 >> {:;n:} :}
                {:^;:}{:hash; {:;__test-nts:} :}
                {:^;:}{:lang; :}
                {:^;:}{:moveto; <div3 >> 3{:;__test-nts:} :}
                {:^;:}{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}
                {:^;:}{:rand; 1..1 :}
                {:^;:}{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}
                {:^;:}{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}
                {:^;:}{:trans; {:trans; Hello nts :} :}
            :}
        :}
    "#
    .trim();

        let expected = "2<div id=\"\" class=\"neutral-fetch-auto \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>one|two|threeHello ntsHello nts0one1two2threetrueHello ntsHello ntscontainsis definedelsentsis filled01234567895c96e4f24ce6e234e6bd4df066748030en{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}1Hello onentsHello";

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.set_src_str(src);
        let result_write = template.render();
        let result_write_parts: Vec<&str> = result_write.split("::--::").collect();
        assert!(!template.has_error());
        assert_eq!(
            result_write_parts[0],
            "<div1>1nts</div1><div2>2nts</div2><div3>3nts</div3>"
        );
        assert_eq!(result_write_parts[2], expected);
        assert_eq!(result_write_parts[4], expected);
        assert_eq!(result_write_parts[6], expected);

        // we give 1 second for “date” to show a different result in !cache
        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result_read = template.render();
        let result_read_parts: Vec<&str> = result_read.split("::--::").collect();
        assert!(!template.has_error());

        assert_eq!(result_write_parts[0], result_read_parts[0]);

        // if the dates are not different, it has not been read from the cache.
        assert_ne!(result_write_parts[1], result_read_parts[1]);

        assert_eq!(result_write_parts[2], result_read_parts[2]);
        assert_eq!(result_write_parts[3], result_read_parts[3]);
        assert_eq!(result_write_parts[4], result_read_parts[4]);

        // if the dates are not different, it has not been read from the cache.
        assert_ne!(result_write_parts[5], result_read_parts[5]);

        assert_eq!(result_write_parts[6], result_read_parts[6]);
    }

    #[test]
    fn test_bif_cache_nested() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = r#"
        {:^;:}::{:^date; %S :}::
        {:cache; /3/ >>
            {:^;:}::{:^date; %S :}::
            {:!cache;
                {:^;:}::{:^date; %S :}::
                {:cache; /2/ >>
                    {:^;:}::{:^date; %S :}::
                    {:!cache;
                        {:^;:}::{:^date; %S :}::
                    :}
                :}
            :}
        :}
    "#
        .trim();

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.set_src_str(src);
        let result_write = template.render();
        let result_write_parts: Vec<&str> = result_write.split("::").collect();
        assert!(!template.has_error());
        assert_eq!(result_write_parts[0], "");
        assert_eq!(result_write_parts[2], "");
        assert_eq!(result_write_parts[4], "");
        assert_eq!(result_write_parts[6], "");
        assert_eq!(result_write_parts[8], "");
        assert_eq!(result_write_parts[10], "");

        // we give 1 second for “date” to show a different result in !cache
        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result_read = template.render();
        let result_read_parts: Vec<&str> = result_read.split("::").collect();
        assert!(!template.has_error());

        assert_eq!(result_write_parts[0], result_read_parts[0]);
        assert_ne!(result_write_parts[1], result_read_parts[1]);
        assert_eq!(result_write_parts[2], result_read_parts[2]);
        assert_eq!(result_write_parts[3], result_read_parts[3]);
        assert_eq!(result_write_parts[4], result_read_parts[4]);
        assert_ne!(result_write_parts[5], result_read_parts[5]);
        assert_eq!(result_write_parts[6], result_read_parts[6]);
        assert_eq!(result_write_parts[7], result_read_parts[7]);
        assert_eq!(result_write_parts[8], result_read_parts[8]);
        assert_ne!(result_write_parts[9], result_read_parts[9]);
        assert_eq!(result_write_parts[10], result_read_parts[10]);
    }

    #[test]
    fn test_bif_cache_nested_inyect() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = r#"
        {:^;:}::{:^date; %S :}{:;inject:}::
        {:cache; /3/ >>
            {:^;:}::{:^date; %S :}{:;inject:}::
            {:!cache;
                {:^;:}::{:^date; %S :}{:;inject:}::
                {:cache; /2/ >>
                    {:^;:}::{:^date; %S :}{:;inject:}::
                    {:!cache;
                        {:^;:}::{:^date; %S :}{:;inject:}::
                        {:cache; /2/ >>
                            {:^;:}::{:^date; %S :}{:;inject:}::
                            {:!cache;
                                {:^;:}::{:^date; %S :}{:;inject:}::
                            :}
                        :}
                    :}
                :}
            :}
        :}
    "#
        .trim();

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.set_src_str(src);
        let result_write = template.render();
        let result_write_parts: Vec<&str> = result_write.split("::").collect();
        assert!(!template.has_error());
        assert_eq!(result_write_parts[0], "");
        assert_eq!(result_write_parts[2], "");
        assert_eq!(result_write_parts[4], "");
        assert_eq!(result_write_parts[6], "");
        assert_eq!(result_write_parts[8], "");
        assert_eq!(result_write_parts[10], "");
        assert_eq!(result_write_parts[12], "");
        assert_eq!(result_write_parts[14], "");

        // we give 1 second for “date” to show a different result in !cache
        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result_read = template.render();
        let result_read_parts: Vec<&str> = result_read.split("::").collect();
        assert!(!template.has_error());

        assert_eq!(result_write_parts[0], result_read_parts[0]);
        assert_ne!(result_write_parts[1], result_read_parts[1]);
        assert_eq!(result_write_parts[2], result_read_parts[2]);
        assert_eq!(result_write_parts[3], result_read_parts[3]);
        assert_eq!(result_write_parts[4], result_read_parts[4]);
        assert_ne!(result_write_parts[5], result_read_parts[5]);
        assert_eq!(result_write_parts[6], result_read_parts[6]);
        assert_eq!(result_write_parts[7], result_read_parts[7]);
        assert_eq!(result_write_parts[8], result_read_parts[8]);
        assert_ne!(result_write_parts[9], result_read_parts[9]);
        assert_eq!(result_write_parts[10], result_read_parts[10]);
        assert_eq!(result_write_parts[11], result_read_parts[11]);
        assert_eq!(result_write_parts[12], result_read_parts[12]);
        assert_ne!(result_write_parts[13], result_read_parts[13]);
        assert_eq!(result_write_parts[14], result_read_parts[14]);
    }

    #[test]
    fn test_bif_cache_neutral_js() {
        let schema = r#"
    {
        "config": {
            "disable_js": false
        }
    }
    "#
        .trim();

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = r#"
        <!DOCTYPE html>
            <head>
                <title>Title</title>
            </head>
            <body>
                <main>
                    {:fetch; "/url" >>
                        loading...
                    :}
                </main>
            </body>
        </html>
    "#
        .trim();

        let neutral_js = NEUTRAL_JS.to_string();
        let expected = r#"
        <!DOCTYPE html>
            <head>
                <title>Title</title>
            </head>
            <body>
                <main>
                    <div id="" class="neutral-fetch-auto " data-url="/url" data-wrap="">
    loading...
</div>

                </main>
            {}</body>
        </html>
    "#
        .trim()
        .replace("{}", &neutral_js);

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, expected);

        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bif_cache_neutral_js_disable() {
        let schema = r#"
    {
        "config": {
            "disable_js": true
        }
    }
    "#
        .trim();

        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = r#"
        <!DOCTYPE html>
            <head>
                <title>Title</title>
            </head>
            <body>
                <main>
                    {:fetch; "/url" >>
                        loading...
                    :}
                </main>
            </body>
        </html>
    "#
        .trim();

        let expected = r#"
        <!DOCTYPE html>
            <head>
                <title>Title</title>
            </head>
            <body>
                <main>
                    <div id="" class="neutral-fetch-auto " data-url="/url" data-wrap="">
    loading...
</div>
                </main>
            </body>
        </html>
    "#
        .trim();

        // fisrt
        template.merge_schema_str(SCHEMA_CACHE).unwrap();
        template.merge_schema_str(schema).unwrap();
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, expected);

        thread::sleep(time::Duration::from_secs(1));

        // read
        template.set_src_str(src);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, expected);
    }


    #[test]
    fn test_bif_cache_invalid_flag() {
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };

        let src = "<div>{:cache; {:flg; invalid_flag :} /3/ >> nts :}</div>";

        template.set_src_str(src);
        let result = template.render();
        assert!(template.has_error());
        assert_eq!(result, "<div></div>");
    }

}
