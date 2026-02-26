use crate::utils::{get_from_key, is_bool_key};
use serde_json::Value;
use std::env;

pub(crate) struct Shared {
    pub(crate) schema: Value,
    pub(crate) lang: String,
    pub(crate) comments: String,
    pub(crate) bisf_count: u64,
    pub(crate) bisf_max: u64,
    pub(crate) flags: String,
    pub(crate) exit: bool,
    pub(crate) has_error: bool,
    pub(crate) status_code: String,
    pub(crate) status_text: String,
    pub(crate) status_param: String,
    pub(crate) redirect_js: String,
    pub(crate) filter_all: bool,
    pub(crate) filter_bifs: bool,
    pub(crate) cache_prefix: String,
    pub(crate) cache_dir: String,
    pub(crate) cache_on_post: bool,
    pub(crate) cache_on_get: bool,
    pub(crate) cache_on_cookies: bool,
    pub(crate) cache_disable: bool,
    pub(crate) disable_js: bool,
    pub(crate) already_js: bool,
    pub(crate) debug_expire: u64,
    pub(crate) debug_file: String,
    pub(crate) working_dir: String,
}

impl Shared {
    pub(crate) fn new(schema: Value) -> Self {
        let bisf_max = schema["config"]["infinite_loop_max_bifs"].as_u64().unwrap();
        let comments = get_from_key(&schema["config"], "comments");
        let lang = get_from_key(&schema["inherit"]["locale"], "current");
        let filter_all = is_bool_key(&schema["config"], "filter_all");
        let cache_prefix = get_from_key(&schema["config"], "cache_prefix");
        let mut cache_dir = get_from_key(&schema["config"], "cache_dir");
        let working_dir = env::current_dir().unwrap().to_string_lossy().into_owned();
        let cache_on_post = is_bool_key(&schema["config"], "cache_on_post");
        let cache_on_get = is_bool_key(&schema["config"], "cache_on_get");
        let cache_on_cookies = is_bool_key(&schema["config"], "cache_on_cookies");
        let cache_disable = is_bool_key(&schema["config"], "cache_disable");
        let disable_js = is_bool_key(&schema["config"], "disable_js");
        let debug_expire = schema["config"]["debug_expire"].as_u64().unwrap();
        let debug_file = get_from_key(&schema["config"], "debug_file");
        let mut filter_bifs = false;

        if !cache_disable {
            filter_bifs = true;
        }

        if cache_dir.is_empty() {
            cache_dir = env::temp_dir().to_string_lossy().into_owned();
        }

        Shared {
            schema,
            lang,
            comments,
            bisf_count: 0,
            bisf_max,
            flags: String::new(),
            exit: false,
            has_error: false,
            status_code: "200".to_string(),
            status_text: "OK".to_string(),
            status_param: String::new(),
            redirect_js: String::new(),
            filter_all,
            filter_bifs,
            cache_prefix,
            cache_dir,
            cache_on_post,
            cache_on_get,
            cache_on_cookies,
            cache_disable,
            disable_js,
            already_js: false,
            debug_expire,
            debug_file,
            working_dir,
        }
    }
}
