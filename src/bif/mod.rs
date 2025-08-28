//

use chrono::Local;
use crate::{
    json, Value,
    block_parser::BlockInherit,
    constants::*,
    shared::Shared,
    utils::*,
};

mod constants;
mod parse_bif_allow;
mod parse_bif_array;
mod parse_bif_bool;
mod parse_bif_cache;
mod parse_bif_coalesce;
mod parse_bif_code;
mod parse_bif_contains;
mod parse_bif_count;
mod parse_bif_data;
mod parse_bif_date;
mod parse_bif_declare;
mod parse_bif_defined;
mod parse_bif_each;
mod parse_bif_else;
mod parse_bif_eval;
mod parse_bif_exit;
mod parse_bif_fetch;
mod parse_bif_filled;
mod parse_bif_flg;
mod parse_bif_for;
mod parse_bif_hash;
mod parse_bif_include;
mod parse_bif_join;
mod parse_bif_lang;
mod parse_bif_locale;
mod parse_bif_moveto;
mod parse_bif_neutral;
mod parse_bif_param;
mod parse_bif_rand;
mod parse_bif_redirect;
mod parse_bif_replace;
mod parse_bif_same;
mod parse_bif_snippet;
mod parse_bif_sum;
mod parse_bif_trans;
mod parse_bif_unknown;
mod parse_bif_var;
mod parse_bif_obj;

mod exec_python;
pub use exec_python::PythonExecutor;

pub(crate) struct BifError {
    pub(crate) msg: String,
    pub(crate) file: String,
    pub(crate) name: String,
    pub(crate) src: String,
}

pub(crate) struct Bif<'a> {
    pub(crate) raw: &'a str,
    pub(crate) shared: &'a mut Shared,
    pub(crate) inherit: &'a mut BlockInherit,
    pub(crate) src: String,
    pub(crate) name: String,
    pub(crate) alias: String,
    pub(crate) code: String,
    pub(crate) params: String,
    pub(crate) flags: String,
    pub(crate) mod_filter: bool,
    pub(crate) mod_negate: bool,
    pub(crate) mod_upline: bool,
    pub(crate) mod_scope: bool,
    pub(crate) file_path: String,
    pub(crate) dir: String,
    pub(crate) out: String,
    pub(crate) only: &'a str,
    _none: &'a str,
}

impl<'a> Bif<'a> {
    pub(crate) fn new(
        raw_source: &'a str,
        shared: &'a mut Shared,
        inherit: &'a mut BlockInherit,
        only: &'a str,
    ) -> Self {
        shared.bisf_count += 1;
        let count = shared.bisf_count;
        inherit.bif_count = shared.bisf_count;

        if count > shared.bisf_max {
            panic!(
                "Infinite loop? {} bifs of {} max have been created.",
                shared.bisf_max, count
            );
        }

        Bif {
            raw: raw_source, // should not be modified
            shared,
            inherit,
            src: String::new(),
            name: String::new(),
            alias: String::new(),
            code: String::new(),
            params: String::new(),
            flags: String::new(),
            mod_filter: false,
            mod_negate: false,
            mod_upline: false,
            mod_scope: false,
            file_path: String::new(),
            dir: String::new(),
            out: String::new(),
            only,
            _none: "",
        }
    }

    // Divides the bif into its parts and executes the bif parse function.
    pub(crate) fn parse(&mut self) -> String {
        let bif = strip_prefix_suffix(self.raw, BIF_OPEN, BIF_CLOSE);
        let result;

        if let Some((name, src)) = bif.split_once(BIF_NAME) {
            self.name = name.to_string();
            self.src = src.trim().to_string();
        } else {
            if !self.only.is_empty() {
                return self.raw.to_string();
            }

            let show_error = self.shared.schema["config"]["error"]["show"]
                .as_bool()
                .unwrap();
            let error_line = format!("The delimiter was not found: {}", self.raw);
            let error_line = error_line.replace(['\n', '\r'], " ");

            if let Some(Value::Array(errors)) = self.shared.schema.get_mut("__error") {
                errors.push(json!(error_line));
            }

            if show_error {
                eprintln!("{}", error_line);
            }

            self.shared.has_error = true;

            return EMPTY_STRING;
        }

        if !self.only.is_empty() && !self.name.contains(self.only) && !self.inherit.in_only {
            return self.raw.to_string();
        }

        self.name = self.set_modifiers();
        self.alias = self.name.clone();
        self.inherit.in_only = true;

        // exec the function of each bif
        match &self.name[..] {
            "" => result = self.parse_bif_var(),
            "allow" => result = self.parse_bif_allow(),
            "array" => result = self.parse_bif_array(),
            "bool" => result = self.parse_bif_bool(),
            "cache" => result = self.parse_bif_cache(),
            "coalesce" => result = self.parse_bif_coalesce(),
            "code" => result = self.parse_bif_code(),
            "contains" => result = self.parse_bif_contains(),
            "count" => result = self.parse_bif_count(),
            "data" => result = self.parse_bif_data(),
            "date" => result = self.parse_bif_date(),
            "declare" => result = self.parse_bif_declare(),
            "defined" => result = self.parse_bif_defined(),
            "each" => result = self.parse_bif_each(),
            "else" => result = self.parse_bif_else(),
            "eval" => result = self.parse_bif_eval(),
            "exit" => result = self.parse_bif_exit(),
            "fetch" => result = self.parse_bif_fetch(),
            "filled" => result = self.parse_bif_filled(),
            "flg" => result = self.parse_bif_flg(),
            "for" => result = self.parse_bif_for(),
            "hash" => result = self.parse_bif_hash(),
            "include" => result = self.parse_bif_include(),
            "join" => result = self.parse_bif_join(),
            "lang" => result = self.parse_bif_lang(),
            "locale" => result = self.parse_bif_locale(),
            "moveto" => result = self.parse_bif_moveto(),
            "neutral" => result = self.parse_bif_neutral(),
            "param" => result = self.parse_bif_param(),
            "rand" => result = self.parse_bif_rand(),
            "redirect" => result = self.parse_bif_redirect(),
            "replace" => result = self.parse_bif_replace(),
            "same" => result = self.parse_bif_same(),
            "snippet" => result = self.parse_bif_snippet(),
            "snip" => result = self.parse_bif_snippet(),
            "sum" => result = self.parse_bif_sum(),
            "trans" => result = self.parse_bif_trans(),
            "obj" => result = self.parse_bif_obj(),
            _ => result = self.parse_bif_unknown(),
        }

        match result {
            Ok(()) => (),
            Err(e) => {
                let show_error = self.shared.schema["config"]["error"]["show"]
                    .as_bool()
                    .unwrap();

                let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                let error_line = format!(
                    "[{}] Error ({}) {} file~: ({}) src: {}",
                    datetime,
                    e.name,
                    e.msg,
                    e.file,
                    e.src
                );

                let error_line = error_line.replace(['\n', '\r'], " ");

                if let Some(Value::Array(errors)) = self.shared.schema.get_mut("__error") {
                    errors.push(json!(error_line));
                }

                if show_error {
                    eprintln!("{}", error_line);
                }

                self.shared.has_error = true;
            }
        }

        self.inherit.last_bif_out = !self.out.is_empty();
        self.inherit.last_coalesce_out = self.inherit.last_bif_out;

        if self.mod_upline {
            self.out = BACKSPACE.to_string() + &self.out;
            self.out.trim().to_string()
        } else {
            self.out.trim().to_string()
        }
    }

    //  Determines which modifiers are being used
    //
    //    .------ modifier
    //    |
    //    v
    //  {:!snippet; ...
    //
    pub(crate) fn set_modifiers(&mut self) -> String {
        let mut index = 0;
        while index < self.name.len() {
            let start = &self.name[index..index + 1];
            if start == BIF_MOD_FILTER
                || start == BIF_MOD_NEGATE
                || start == BIF_MOD_UPLINE
                || start == BIF_MOD_SCOPE
            {
                match start {
                    BIF_MOD_FILTER => self.mod_filter = true,
                    BIF_MOD_NEGATE => self.mod_negate = true,
                    BIF_MOD_UPLINE => self.mod_upline = true,
                    BIF_MOD_SCOPE => self.mod_scope = true,
                    _ => unreachable!(),
                }
                index += 1;
            } else {
                break;
            }
        }

        self.name[index..].to_string()
    }

    // Get key from schema data o local data
    //
    // {
    //     "config": {},
    //     "inherit": {},
    //     "data": {}  <------------ schema data get from
    //     "__indir": {
    //          "X": {
    //             "data": {} <----- local data get from
    //     ...
    // }
    pub(crate) fn get_data(&self, name: &str) -> String {
        if name.starts_with("local::") {
            let local_name = name.strip_prefix("local::").unwrap_or(name);
            get_from_key(
                &self.shared.schema["__indir"][&self.inherit.indir]["data"],
                local_name,
            )
        } else {
            get_from_key(&self.shared.schema["data"], name)
        }
    }

    // Set key to schema data
    //
    // {
    //     "config": {},
    //     "inherit": {},
    //     "data": {}  <-------- set to
    // }
    pub(crate) fn set_data(&mut self, name: &str, value: &str) {
        self.shared.schema["data"][name] = json!(value);
    }

    // Get key from schema locale, an indirection is used instead of its initial position
    // {
    //     "config": {},
    //     "inherit": {
    //     "locale": { ------------------.
    //        "current": "en",           |
    //        "trans": {                 |
    //           "es": {}                |
    //         }                         | moved on init Template
    //     },                            |
    //     "data": {},                   |
    //     "__indir": {                  |
    //          "X": {                   |
    //             "locale": { <---------·
    //                 "trans": {
    //                     "es": {} <----- get from
    //     ...
    // }
    pub(crate) fn get_trans(&self, text: &str) -> String {
        get_from_key(
            &self.shared.schema["__indir"][&self.inherit.indir]["locale"]["trans"]
                [&self.shared.lang],
            text,
        )
    }

    /*
        dynamic evaluation

        This is not allowed: {:;{:;refvarname:}:}
        Use instead: {:; {:allow; allowed >> {:;refvarname:} :} :}
    */
    pub(crate) fn contains_allow(&self, source: &str) -> bool {
        for allow in BIF_ALLOWED {
            if source.contains(allow) {
                return true;
            }
        }

        let source = &remove_comments(source);
        !(source.starts_with(BIF_VAR) && source.ends_with(BIF_CLOSE))
    }

    // Split params/code and parse params if parse is true.
    // It is possible that it has no parameters, in which case
    // it is all code and the parameters are an empty string.
    // To set flags, parameters are required.
    //
    //                   .------------------------------> params
    //                   |       .----------------------> separator
    //                   |       |
    //                   |       |                 .----> code
    //                   |       |                 |
    //                   v       v                 v
    //              ------------ -- ------------------------------
    //  {:!snippet; snippet_name >> <div>... {:* ... *:} ...</div> :}
    pub(crate) fn extract_params_code(&mut self, parse: bool) -> bool {
        let position = get_code_position(&self.src);
        let has_code: bool = position.is_some();

        if has_code {
            let code_pos = position.unwrap();
            self.params = self.src[0..code_pos].trim().to_string();
            self.code = self.src[code_pos + BIF_CODE.len()..].trim().to_string();
        } else {
            self.params = EMPTY_STRING;
            self.code = self.src.trim().to_string();
        }

        if parse && self.params.contains(BIF_OPEN) {
            self.shared.flags = EMPTY_STRING;
            self.params = new_child_parse!(self, &self.params, false);
            self.flags = self.shared.flags.clone();
        }

        has_code
    }

    // Extract bif arguments.
    //
    //          .-- arg 0 empty string
    //          | .-- arg 1
    //          | |    .-- arg 2
    //          | |    |     .-- arg n
    //          | |    |     |
    //          v v    v     v
    // {:fetch; |url|event| ... >> ... :}
    // result: ["", "url", "event", ""]
    pub(crate) fn extract_args(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let delim;
        if let Some(first_char) = self.params.chars().next() {
            delim = first_char;
        } else {
            return vec!["".to_string()];
        }

        let mut parts = self.params.split(delim);

        while let Some(ref mut part) = parts.next() {
            let mut arg = part.to_string();

            if arg.contains(BIF_OPEN) {
                arg = new_child_parse!(self, &arg, false);
            }

            result.push(arg);
        }

        result
    }

    pub(crate) fn bif_error(&self, msg: &str) -> BifError {
        BifError {
            msg: msg.to_string(),
            name: self.alias.clone(),
            file: self.inherit.current_file.clone(),
            src: self.raw.to_string(),
        }
    }

}
