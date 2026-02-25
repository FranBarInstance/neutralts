use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use serde_json::{json, Value};
use regex::Regex;
use crate::{
    constants::*,
    default_json::*,
    utils::*,
    shared::Shared,
    block_parser::BlockParser,
    block_parser::BlockInherit
};

pub struct Template<'a> {
    raw: String,
    file_path: &'a str,
    schema: Value,
    shared: Shared,
    time_start: Instant,
    time_elapsed: Duration,
    out: String,
}

/// A struct representing a template that can be rendered.
///
/// This struct is used to handle the rendering of templates.
impl<'a> Template<'a> {
    /// Constructs a new `Template` instance with default settings.
    ///
    /// It allows you to set up a template and schema with different types.
    pub fn new() -> Result<Self, String> {
        let default_schema: Value = match serde_json::from_str(DEFAULT) {
            Ok(value) => value,
            Err(_) => return Err("const DEFAULT is not a valid JSON string".to_string()),
        };
        let shared = Shared::new(default_schema.clone());

        Ok(Template {
            raw: String::new(),
            file_path: "",
            schema: default_schema,
            shared,
            time_start: Instant::now(),
            time_elapsed: Instant::now().elapsed(),
            out: String::new(),
        })
    }

    /// Constructs a new `Template` instance from a file path and a JSON schema.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to the path of the file containing the template content.
    /// * `schema` - A JSON value representing the custom schema to be used with the template.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Template` instance or an error message if:
    /// - The file cannot be read.
    pub fn from_file_value(file_path: &'a str, schema: Value) -> Result<Self, String> {
        let raw: String = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot be read: {}", file_path);
                return Err(e.to_string());
            }
        };
        let mut default_schema: Value = match serde_json::from_str(DEFAULT) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Internal error in const DEFAULT {}, line: {}", file!(), line!());
                return Err("const DEFAULT is not a valid JSON string".to_string());
            }
        };

        update_schema_owned(&mut default_schema, schema);
        let shared = Shared::new(default_schema.clone());

        Ok(Template {
            raw,
            file_path,
            schema: default_schema,
            shared,
            time_start: Instant::now(),
            time_elapsed: Instant::now().elapsed(),
            out: String::new(),
        })
    }

    /// Sets the source path of the template.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to the path of the file containing the template content.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if the file cannot be read
    pub fn set_src_path(&mut self, file_path: &'a str) -> Result<(), String> {
        self.file_path = file_path;
        self.raw = match fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot be read: {}", file_path);
                return Err(e.to_string());
            }
        };

        Ok(())
    }

    /// Sets the content of the template from a string.
    ///
    /// # Arguments
    ///
    /// * `source` - A reference to the new string content to be set as the raw content.
    pub fn set_src_str(&mut self, source: &str) {
        self.raw = source.to_string();
    }

    /// Merges the schema from a file with the current template schema.
    ///
    /// # Arguments
    ///
    /// * `schema_path` - A reference to the path of the file containing the schema content.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if:
    /// - The file cannot be read.
    /// - The file's content is not a valid JSON string.
    pub fn merge_schema_path(&mut self, schema_path: &str) -> Result<(), String> {
        let schema_str: String = match fs::read_to_string(schema_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Cannot be read: {}", schema_path);
                return Err(e.to_string());
            }
        };
        let schema_value: Value = match serde_json::from_str(&schema_str) {
            Ok(value) => value,
            Err(_) => {
                return Err("Is not a valid JSON file".to_string());
            }
        };
        update_schema(&mut self.schema, &schema_value);

        Ok(())
    }

    /// Merges the schema from a JSON string with the current template schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - A reference to the JSON string of the schema content.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if:
    /// - The file's content is not a valid JSON string.
    pub fn merge_schema_str(&mut self, schema: &str) -> Result<(), String> {
        let schema_value: Value = match serde_json::from_str(schema) {
            Ok(value) => value,
            Err(_) => {
                return Err("Is not a valid JSON string".to_string());
            }
        };
        update_schema(&mut self.schema, &schema_value);

        Ok(())
    }

    /// Merges the provided JSON value with the current schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - The JSON Value to be merged with the current schema.
    pub fn merge_schema_value(&mut self, schema: Value) {
        update_schema_owned(&mut self.schema, schema);
    }

    /// Constructs a new `Template` instance from a file path and MessagePack schema bytes.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to the path of the file containing the template content.
    /// * `bytes` - A byte slice containing the MessagePack schema.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Template` instance or an error message if:
    /// - The template file cannot be read.
    /// - The MessagePack data is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use neutralts::Template;
    /// let bytes = vec![129, 164, 100, 97, 116, 97, 129, 163, 107, 101, 121, 165, 118, 97, 108, 117, 101];
    /// let template = Template::from_file_msgpack("template.ntpl", &bytes).unwrap();
    /// ```
    pub fn from_file_msgpack(file_path: &'a str, bytes: &[u8]) -> Result<Self, String> {
        let schema: Value = if bytes.is_empty() {
            json!({})
        } else {
            match rmp_serde::from_slice(bytes) {
                Ok(v) => v,
                Err(e) => return Err(format!("Invalid MessagePack data: {}", e)),
            }
        };

        Self::from_file_value(file_path, schema)
    }

    /// Merges the schema from a MessagePack file with the current template schema.
    ///
    /// # Arguments
    ///
    /// * `msgpack_path` - A reference to the path of the file containing the MessagePack schema.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if:
    /// - The file cannot be read.
    /// - The file's content is not a valid MessagePack.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use neutralts::Template;
    /// let mut template = Template::new().unwrap();
    /// template.merge_schema_msgpack_path("extra_data.msgpack").unwrap();
    /// ```
    pub fn merge_schema_msgpack_path(&mut self, msgpack_path: &str) -> Result<(), String> {
        let msgpack_data = match fs::read(msgpack_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Cannot be read: {}", msgpack_path);
                return Err(e.to_string());
            }
        };

        self.merge_schema_msgpack(&msgpack_data)
    }

    /// Merges the schema from MessagePack bytes with the current template schema.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte slice containing the MessagePack schema.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an error message if:
    /// - The bytes are not a valid MessagePack.
    ///
    /// # Example
    ///
    /// ```
    /// use neutralts::Template;
    /// let mut template = Template::new().unwrap();
    /// let bytes = vec![129, 164, 100, 97, 116, 97, 129, 163, 107, 101, 121, 165, 118, 97, 108, 117, 101];
    /// template.merge_schema_msgpack(&bytes).unwrap();
    /// ```
    pub fn merge_schema_msgpack(&mut self, bytes: &[u8]) -> Result<(), String> {
        let schema_value: Value = match rmp_serde::from_slice(bytes) {
            Ok(value) => value,
            Err(e) => {
                return Err(format!("Is not a valid MessagePack data: {}", e));
            }
        };
        update_schema_owned(&mut self.schema, schema_value);

        Ok(())
    }

    /// Renders the template content.
    ///
    /// This function initializes the rendering process.
    /// The resulting output is returned as a string.
    ///
    /// # Returns
    ///
    /// The rendered template content as a string.
    pub fn render(&mut self) -> String {
        let inherit = self.init_render();
        self.out = BlockParser::new(&mut self.shared, inherit.clone()).parse(&self.raw, "");

        while self.out.contains("{:!cache;") {
            let out;
            out = BlockParser::new(&mut self.shared, inherit.clone()).parse(&self.out, "!cache");
            self.out = out;
        }

        self.ends_render();

        self.out.clone()
    }

    // Restore vars for render
    fn init_render(&mut self) -> BlockInherit {
        self.time_start = Instant::now();
        self.shared = Shared::new(self.schema.clone());

        if self.shared.comments.contains("remove") {
            self.raw = remove_comments(&self.raw);
        }

        // init inherit
        let mut inherit = BlockInherit::new();
        let indir = inherit.create_block_schema(&mut self.shared);
        self.shared.schema["__moveto"] = json!({});
        self.shared.schema["__error"] = json!([]);
        self.shared.schema["__indir"] = json!({});
        self.shared.schema["__indir"][&indir] = self.shared.schema["inherit"].clone();
        inherit.current_file = self.file_path.to_string();

        // Escape CONTEXT values
        filter_value(&mut self.shared.schema["data"]["CONTEXT"]);

        // Escape CONTEXT keys names
        filter_value_keys(&mut self.shared.schema["data"]["CONTEXT"]);

        if !self.file_path.is_empty() {
            let path = Path::new(&self.file_path);

            if let Some(parent) = path.parent() {
                inherit.current_dir = parent.display().to_string();
            }
        } else {
            inherit.current_dir = self.shared.working_dir.clone();
        }

        if !self.shared.debug_file.is_empty() {
            eprintln!("WARNING: config->debug_file is not empty: {} (Remember to remove this in production)", self.shared.debug_file);
        }

        inherit
    }

    // Rendering ends
    fn ends_render(&mut self) {
        self.set_moveto();
        self.replacements();
        self.set_status_code();
        self.time_elapsed = self.time_start.elapsed();
    }

    fn set_status_code(&mut self) {
        let status_code = self.shared.status_code.as_str();

        if ("400"..="599").contains(&status_code) {
            self.out = format!("{} {}", self.shared.status_code, self.shared.status_text);

            return;
        }

        if status_code == "301"
            || status_code == "302"
            || status_code == "303"
            || status_code == "307"
            || status_code == "308"
        {
            self.out = format!(
                "{} {}\n{}",
                self.shared.status_code, self.shared.status_text, self.shared.status_param
            );

            return;
        }

        if !self.shared.redirect_js.is_empty() {
            self.out = self.shared.redirect_js.clone();
        }
    }

    fn set_moveto(&mut self) {
        if let Value::Object(data_map) = &self.shared.schema["__moveto"] {
            for (_key, value) in data_map {
                if let Value::Object(inner_map) = value {
                    for (inner_key, inner_value) in inner_map {
                        let mut tag;

                        // although it should be "<tag" or "</tag" it also supports
                        // "tag", "/tag", "<tag>" and "</tag>
                        if !inner_key.starts_with("<") {
                            tag = format!("<{}", inner_key);
                        } else {
                            tag = inner_key.to_string();
                        }
                        if tag.ends_with(">") {
                            tag = tag[..tag.len() - 1].to_string();
                        }

                        // if it does not find it, it does nothing
                        let position = find_tag_position(&self.out, &tag);
                        if let Some(pos) = position {
                            let mut insert = inner_value.as_str().unwrap().to_string();
                            insert = insert.to_string();
                            self.out.insert_str(pos, &insert);
                        }
                    }
                }
            }
        }
    }

    fn replacements(&mut self) {
        if self.out.contains(BACKSPACE) {
            lazy_static::lazy_static! {
                static ref RE: Regex = Regex::new(&format!(r"\s*{}", BACKSPACE)).expect("Failed to create regex with constant pattern");
            }
            if let std::borrow::Cow::Owned(s) = RE.replace_all(&self.out, "") {
                self.out = s;
            }
        }

        // UNPRINTABLE should be substituted after BACKSPACE
        if self.out.contains(UNPRINTABLE) {
            self.out = self.out.replace(UNPRINTABLE, "");
        }
    }

    /// Retrieves the status code.
    ///
    /// The status code is "200" unless "exit", "redirect" is used or the
    /// template contains a syntax error, which will return a status code
    /// of "500". Although the codes are numeric, a string is returned.
    ///
    /// # Returns
    ///
    /// A reference to the status code as a string.
    pub fn get_status_code(&self) -> &String {
        &self.shared.status_code
    }

    /// Retrieves the status text.
    ///
    /// It will correspond to the one set by the HTTP protocol.
    ///
    /// # Returns
    ///
    /// A reference to the status text as a string.
    pub fn get_status_text(&self) -> &String {
        &self.shared.status_text
    }

    /// Retrieves the status parameter.
    ///
    /// Some statuses such as 301 (redirect) may contain additional data, such
    /// as the destination URL, and in similar cases “param” will contain
    /// that value.
    ///
    /// # Returns
    ///
    /// A reference to the status parameter as a string.
    pub fn get_status_param(&self) -> &String {
        &self.shared.status_param
    }

    /// Checks if there is an error.
    ///
    /// If any error has occurred, in the parse or otherwise, it will return true.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether there is an error.
    pub fn has_error(&self) -> bool {
        self.shared.has_error
    }

    /// Get bifs errors list
    ///
    /// # Returns
    ///
    /// * `Value`: A clone of the value with the list of errors in the bifs during rendering.
    pub fn get_error(&self) -> Value {
        self.shared.schema["__error"].clone()
    }

    /// Retrieves the time duration for template rendering.
    ///
    /// # Returns
    ///
    /// The time duration elapsed .
    pub fn get_time_duration(&self) -> Duration {
        let duration: std::time::Duration = self.time_elapsed;

        duration
    }
}
