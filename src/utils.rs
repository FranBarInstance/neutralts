
use serde_json::Value;
use crate::constants::*;

/// Merges two JSON schemas represented as `serde_json::Value`.
///
/// This function performs a recursive merge between two JSON objects.
/// If an object has common keys, the values are merged recursively.
/// If the value is not an object, it is directly overwritten.
///
/// # Arguments
///
/// * `a` - A mutable reference to the first JSON object (`serde_json::Value::Object`).
/// * `b` - A reference to the second JSON object (`serde_json::Value::Object`) that will be merged with the first.
///
/// # Example
///
/// ```text
/// use serde_json::{json, Value};
///
/// let mut schema1 = json!({
///     "name": "John",
///     "age": 30,
/// });
///
/// let schema2 = json!({
///     "age": 31,
///     "city": "New York"
/// });
///
/// merge_schema(&mut schema1, &schema2);
/// assert_eq!(schema1, json!({
///     "name": "John",
///     "age": 31,
///     "city": "New York"
/// }));
/// ```
pub fn merge_schema(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            for (k, v) in b_map {
                if let Some(va) = a_map.get_mut(k) {
                    merge_schema(va, v);
                } else {
                    a_map.insert(k.clone(), v.clone());
                }
            }
        }
        (a, b) => *a = b.clone(),
    }
}

/// Merge schema and update some keys
///
/// This is a thin wrapper around `merge_schema` that additionally:
/// 1. Copies the value of the header key `requested-with-ajax` (all lower-case) into the
///    variants `Requested-With-Ajax` (Pascal-Case) and `REQUESTED-WITH-AJAX` (upper-case),
///    or vice-versa, depending on which variant is present in the incoming schema.
/// 2. Overwrites the top-level `version` field with the compile-time constant `VERSION`.
///
/// The three header variants are created so that downstream code can read the header
/// regardless of the casing rules enforced by the environment (HTTP servers, proxies, etc.).
///
/// # Arguments
/// * `a` – the target `Value` (must be an `Object`) that will receive the merge result.
/// * `b` – the source `Value` (must be an `Object`) whose contents are merged into `a`.
///
pub fn update_schema(a: &mut Value, b: &Value) {
    merge_schema(a, b);

    // Different environments may ignore or add capitalization in headers
    let headers = &b["data"]["CONTEXT"]["HEADERS"];
    if headers.get("requested-with-ajax").is_some() {
        a["data"]["CONTEXT"]["HEADERS"]["Requested-With-Ajax"] = b["data"]["CONTEXT"]["HEADERS"]["requested-with-ajax"].clone();
        a["data"]["CONTEXT"]["HEADERS"]["REQUESTED-WITH-AJAX"] = b["data"]["CONTEXT"]["HEADERS"]["requested-with-ajax"].clone();
    } else if headers.get("Requested-With-Ajax").is_some() {
        a["data"]["CONTEXT"]["HEADERS"]["requested-with-ajax"] = b["data"]["CONTEXT"]["HEADERS"]["Requested-With-Ajax"].clone();
        a["data"]["CONTEXT"]["HEADERS"]["REQUESTED-WITH-AJAX"] = b["data"]["CONTEXT"]["HEADERS"]["Requested-With-Ajax"].clone();
    } else if headers.get("REQUESTED-WITH-AJAX").is_some() {
        a["data"]["CONTEXT"]["HEADERS"]["requested-with-ajax"] = b["data"]["CONTEXT"]["HEADERS"]["REQUESTED-WITH-AJAX"].clone();
        a["data"]["CONTEXT"]["HEADERS"]["Requested-With-Ajax"] = b["data"]["CONTEXT"]["HEADERS"]["REQUESTED-WITH-AJAX"].clone();
    }

    // Update version
    a["version"] = VERSION.into();
}

/// Extract same level blocks positions.
///
/// ```text
///
///                  .-----> .-----> {:code:
///                  |       |           {:code: ... :}
///                  |       |           {:code: ... :}
///                  |       |           {:code: ... :}
///  Level block --> |       ·-----> :}
///                  |        -----> {:code: ... :}
///                  |       .-----> {:code:
///                  |       |           {:code: ... :}
///                  ·-----> ·-----> :}
///
/// # Arguments
///
/// * `raw_source` - A string slice containing the template source text.
///
/// # Returns
///
/// * `Ok(Vec<(usize, usize)>)`: A vector of tuples representing the start and end positions of each extracted block.
/// * `Err(usize)`: An error position if there are unmatched closing tags or other issues
/// ```
pub fn extract_blocks(raw_source: &str) -> Result<Vec<(usize, usize)>, usize> {
    let mut blocks = Vec::new();
    let bytes = raw_source.as_bytes();
    let mut curr_pos: usize = 0;
    let mut open_pos: usize;
    let mut nested = 0;
    let mut nested_comment = 0;
    let len_open = BIF_OPEN_B.len();
    let len_close = BIF_CLOSE_B.len();
    let len_src = bytes.len();

    while let Some(pos) = find_bytes(bytes, BIF_OPEN_B, curr_pos) {
        curr_pos = pos + len_open;
        open_pos = pos;

        // It is important to extract the comments first because they may have bif commented,
        // we avoid that they are detected as valid and other errors.
        if bytes[curr_pos] == BIF_COMMENT_B {
            while let Some(pos) = find_bytes(bytes, BIF_DELIM_B, curr_pos) {
                curr_pos = pos;

                if curr_pos >= len_src {
                    break;
                }

                if bytes[curr_pos - 1] == BIF_OPEN0 && bytes[curr_pos + 1] == BIF_COMMENT_B  {
                    nested_comment += 1;
                    curr_pos += 1;
                    continue;
                }
                if nested_comment > 0 && bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                    nested_comment -= 1;
                    curr_pos += 1;
                    continue;
                }
                if bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                    curr_pos += len_close;
                    blocks.push((open_pos, curr_pos));
                    break;
                } else {
                    curr_pos += 1;
                }
            }

            continue;
        }

        while let Some(pos) = find_bytes(bytes, BIF_DELIM_B, curr_pos) {
            curr_pos = pos;

            if curr_pos >= len_src {
                break;
            }

            if bytes[curr_pos - 1] == BIF_OPEN0 {
                nested += 1;
                curr_pos += 1;
                continue;
            }
            if nested > 0 && bytes[curr_pos + 1] == BIF_CLOSE1 {
                nested -= 1;
                curr_pos += 1;
                continue;
            }
            if bytes[curr_pos + 1] == BIF_CLOSE1 {
                curr_pos += len_close;
                blocks.push((open_pos, curr_pos));
                break;
            } else {
                curr_pos += 1;
            }
        }
    }

    // Search BIF_CLOSE in the blocks that are not bif, given that we start looking
    // for BIF_OPEN all these keys are found, if anything is left is BIF_CLOSE
    let mut prev_end = 0;
    for (start, end) in &blocks {
        if let Some(error_pos) = find_bytes(&bytes[prev_end..*start], BIF_CLOSE_B, 0) {
            return Err(error_pos + prev_end);
        }
        prev_end = *end;
    }

    let rest = if curr_pos == 0 { 0 } else { curr_pos - 1 };
    if let Some(error_pos) = find_bytes(bytes, BIF_CLOSE_B, rest) {
        return Err(error_pos);
    }

    Ok(blocks)
}

fn find_bytes(bytes: &[u8], substring: &[u8], start_pos: usize) -> Option<usize> {
    let bytes_len = bytes.len();
    let subs_len = substring.len();

    if start_pos >= bytes_len || substring.is_empty() || start_pos + subs_len > bytes_len  {
        return None;
    }

    (start_pos..=bytes_len.saturating_sub(subs_len)).find(|&i| &bytes[i..i + subs_len] == substring)
}

/// Removes a prefix and suffix from a string slice.
///
/// # Arguments
///
/// * `str`: The input string slice.
/// * `prefix`: The prefix to remove.
/// * `suffix`: The suffix to remove.
///
/// # Returns
///
/// * A new string slice with the prefix and suffix removed, or the original string if not found.
pub fn strip_prefix_suffix<'a>(str: &'a str, prefix: &'a str, suffix: &'a str) -> &'a str {
    let start = match str.strip_prefix(prefix) {
        Some(striped) => striped,
        None => return str,
    };
    let end = match start.strip_suffix(suffix) {
        Some(striped) => striped,
        None => return str,
    };

    end
}

/// Retrieves a value from a JSON schema using a specified key.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to retrieve the value from the schema.
///
/// # Returns
///
/// * A `String` containing the retrieved value, or an empty string if the key is not found.
pub fn get_from_key(schema: &Value, key: &str) -> String {
    if let Some(v) = resolve_pointer(schema, key) {
        match v {
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            _ => String::new(),
        }
    } else {
        String::new()
    }
}

/// Checks if the value associated with a key in the schema is considered empty.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is considered empty, otherwise `false`.
pub fn is_empty_key(schema: &Value, key: &str) -> bool {
    if let Some(value) = resolve_pointer(schema, key) {
        match value {
            Value::Object(map) => map.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::String(s) => s.is_empty(),
            Value::Null => true,
            Value::Number(_) => false,
            Value::Bool(_) => false,
        }
    } else {
        true
    }
}

/// Checks if the value associated with a key in the schema is considered a boolean true.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is considered a boolean true, otherwise `false`.
pub fn is_bool_key(schema: &Value, key: &str) -> bool {
    if let Some(value) = resolve_pointer(schema, key) {
        match value {
            Value::Object(obj) => !obj.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::String(s) if s.is_empty() || s == "false" => false,
            Value::String(s) => s.parse::<f64>().ok().map_or(true, |n| n > 0.0),
            Value::Null => false,
            Value::Number(n) => n.as_f64().map_or(false, |f| f > 0.0),
            Value::Bool(b) => *b,
        }
    } else {
        false
    }
}

/// Checks if the value associated with a key in the schema is considered an array.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is an array, otherwise `false`.
pub fn is_array_key(schema: &Value, key: &str) -> bool {
    if let Some(value) = resolve_pointer(schema, key) {
        match value {
            Value::Object(_) => true,
            Value::Array(_) => true,
            _ => false,
        }
    } else {
        false
    }
}

/// Checks if the value associated with a key in the schema is considered defined.
///
/// # Arguments
///
/// * `schema`: A reference to the JSON schema as a `Value`.
/// * `key`: The key used to check the value in the schema.
///
/// # Returns
///
/// * `true` if the value is defined and not null, otherwise `false`.
pub fn is_defined_key(schema: &Value, key: &str) -> bool {
    match resolve_pointer(schema, key) {
        Some(value) => !value.is_null(),
        None => false,
    }
}

/// Helper function to resolve a pointer-like key (e.g., "a->b->0") in a JSON Value.
fn resolve_pointer<'a>(schema: &'a Value, key: &str) -> Option<&'a Value> {
    if !key.contains(BIF_ARRAY) && !key.contains('/') {
        return schema.get(key);
    }

    let mut current = schema;
    for part in key.split(BIF_ARRAY) {
        if part.contains('/') {
            for subpart in part.split('/') {
                if subpart.is_empty() {
                    continue;
                }
                current = match current {
                    Value::Object(map) => map.get(subpart)?,
                    Value::Array(arr) => {
                        let idx = subpart.parse::<usize>().ok()?;
                        arr.get(idx)?
                    }
                    _ => return None,
                };
            }
        } else {
            if part.is_empty() {
                continue;
            }
            current = match current {
                Value::Object(map) => map.get(part)?,
                Value::Array(arr) => {
                    let idx = part.parse::<usize>().ok()?;
                    arr.get(idx)?
                }
                _ => return None,
            };
        }
    }
    Some(current)
}

/// Finds the position of the first occurrence of BIF_CODE_B in the source string,
/// but only when it is not inside any nested brackets.
///
/// ```text
///                   .------------------------------> params
///                   |       .----------------------> this
///                   |       |
///                   |       |                 .----> code
///                   |       |                 |
///                   v       v                 v
///              ------------ -- ------------------------------
///  {:!snippet; snippet_name >> <div>... {:* ... *:} ...</div> :}
pub fn get_code_position(src: &str) -> Option<usize> {
    let mut level = 0;
    src.as_bytes()
        .windows(2)
        .enumerate()
        .find(|&(_, window)| match window {
            x if x == BIF_OPEN_B => {
                level += 1;
                false
            }
            x if x == BIF_CLOSE_B => {
                level -= 1;
                false
            }
            x if x == BIF_CODE_B && level == 0 => true,
            _ => false,
        })
        .map(|(i, _)| i)
}

/// Removes comments from the template source.
pub fn remove_comments(raw_source: &str) -> String {
    let mut result = String::new();
    let mut blocks = Vec::new();
    let bytes = raw_source.as_bytes();
    let mut curr_pos: usize = 0;
    let mut open_pos: usize;
    let mut nested_comment = 0;
    let len_open = BIF_OPEN_B.len();
    let len_close = BIF_CLOSE_B.len();
    let len_src = bytes.len();

    while let Some(pos) = find_bytes(bytes, BIF_COMMENT_OPEN_B, curr_pos) {
        curr_pos = pos + len_open;
        open_pos = pos;

        while let Some(pos) = find_bytes(bytes, BIF_DELIM_B, curr_pos) {
            curr_pos = pos;

            if curr_pos >= len_src {
                break;
            }

            if bytes[curr_pos - 1] == BIF_OPEN0 && bytes[curr_pos + 1] == BIF_COMMENT_B  {
                nested_comment += 1;
                curr_pos += 1;
                continue;
            }
            if nested_comment > 0 && bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                nested_comment -= 1;
                curr_pos += 1;
                continue;
            }
            if bytes[curr_pos + 1] == BIF_CLOSE1 && bytes[curr_pos - 1] == BIF_COMMENT_B {
                curr_pos += len_close;
                blocks.push((open_pos, curr_pos));
                break;
            } else {
                curr_pos += 1;
            }
        }

    }

    let mut prev_end = 0;
    for (start, end) in &blocks {
        result.push_str(&raw_source[prev_end..*start]);
        prev_end = *end;
    }
    result.push_str(&raw_source[curr_pos..]);

    result
}

/// Performs a wildcard matching between a text and a pattern.
///
/// Used in bif "allow" and "declare"
///
/// # Arguments
///
/// * `text`: The text to match against the pattern.
/// * `pattern`: The pattern containing wildcards ('.', '?', '*', '~').
///
/// # Returns
///
/// * `true` if the text matches the pattern, otherwise `false`.
pub fn wildcard_match(text: &str, pattern: &str) -> bool {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    fn match_recursive(text: &[char], pattern: &[char]) -> bool {
        if pattern.is_empty() {
            return text.is_empty();
        }

        let first_char = *pattern.first().unwrap();
        let rest_pattern = &pattern[1..];

        match first_char {
            '\\' => {
                if rest_pattern.is_empty() || text.is_empty() {
                    return false;
                }
                let escaped_char = rest_pattern.first().unwrap();
                match_recursive(&text[1..], &rest_pattern[1..]) && *text.first().unwrap() == *escaped_char
            }
            '.' => {
                match_recursive(text, rest_pattern) || (!text.is_empty() && match_recursive(&text[1..], rest_pattern))
            }
            '?' => {
                !text.is_empty() && match_recursive(&text[1..], rest_pattern)
            }
            '*' => {
                match_recursive(text, rest_pattern) || (!text.is_empty() && match_recursive(&text[1..], pattern))
            }
            '~' => {
                text.is_empty()
            },
            _ => {
                if text.is_empty() || first_char != *text.first().unwrap() {
                    false
                } else {
                    match_recursive(&text[1..], rest_pattern)
                }
            }
        }
    }

    match_recursive(&text_chars, &pattern_chars)
}


/// Finds the position of a tag in the text.
///
/// It is used in the bif "moveto".
///
/// # Arguments
///
/// * `text`: The text to search for the tag.
/// * `tag`: The tag to find.
///
/// # Returns
///
/// * `Some(usize)`: The position of the end of the tag, or None if the tag is not found.
pub fn find_tag_position(text: &str, tag: &str) -> Option<usize> {
    if let Some(start_pos) = text.find(tag) {
        if !tag.starts_with("</") {
            if let Some(end_tag_pos) = text[start_pos..].find('>') {
                return Some(start_pos + end_tag_pos + 1);
            }
        } else {
            return Some(start_pos);
        }
    }

    None
}

/// Escapes special characters in a given input string.
///
/// This function replaces specific ASCII characters with their corresponding HTML entities.
/// It is designed to handle both general HTML escaping and optional escaping of curly braces (`{` and `}`).
///
/// # Arguments
///
/// * `input` - The input string to escape.
/// * `escape_braces` - A boolean flag indicating whether to escape curly braces (`{` and `}`).
///   - If `true`, curly braces are escaped as `&#123;` and `&#125;`.
///   - If `false`, curly braces are left unchanged.
///
/// # Escaped Characters
///
/// The following characters are always escaped:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&#x27;`
/// - `/` → `&#x2F;`
///
/// If `escape_braces` is `true`, the following characters are also escaped:
/// - `{` → `&#123;`
/// - `}` → `&#125;`
///
/// # Examples
///
/// Basic usage without escaping curly braces:
/// ```text
/// let input = r#"Hello, <world> & "friends"! {example}"#;
/// let escaped = escape_chars(input, false);
/// assert_eq!(escaped, r#"Hello, &lt;world&gt; &amp; &quot;friends&quot;! {example}"#);
/// ```
///
/// Escaping curly braces:
/// ```text
/// let input = r#"Hello, <world> & "friends"! {example}"#;
/// let escaped = escape_chars(input, true);
/// assert_eq!(escaped, r#"Hello, &lt;world&gt; &amp; &quot;friends&quot;! &#123;example&#125;"#);
/// ```
pub fn escape_chars(input: &str, escape_braces: bool) -> String {
    let mut result = String::with_capacity(input.len() * 2);

    for c in input.chars() {
        if c.is_ascii() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&#x27;"),
                '/' => result.push_str("&#x2F;"),
                '{' if escape_braces => result.push_str("&#123;"),
                '}' if escape_braces => result.push_str("&#125;"),
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Unescapes HTML entities in a given input string.
///
/// This function is designed specifically to reverse the escaping performed by `escape_chars`.
/// It is not intended to be a general-purpose HTML decoder. It replaces the following HTML
/// entities with their corresponding characters:
/// - `&amp;` → `&`
/// - `&lt;` → `<`
/// - `&gt;` → `>`
/// - `&quot;` → `"`
/// - `&#x27;` → `'`
/// - `&#x2F;` → `/`
///
/// If `escape_braces` is `true`, it also replaces:
/// - `&#123;` → `{`
/// - `&#125;` → `}`
///
/// If an unrecognized entity is encountered, it is left unchanged in the output.
///
/// # Arguments
///
/// * `input` - The input string containing HTML entities to unescape.
/// * `escape_braces` - A boolean flag indicating whether to unescape curly braces (`{` and `}`).
///   - If `true`, `&#123;` and `&#125;` are unescaped to `{` and `}`.
///   - If `false`, `&#123;` and `&#125;` are left unchanged.
///
/// # Examples
///
/// Basic usage:
/// ```text
/// let input = "&lt;script&gt;alert(&quot;Hello &amp; &#x27;World&#x27;&quot;);&lt;/script&gt;";
/// let unescaped = unescape_chars(input, false);
/// assert_eq!(unescaped, r#"<script>alert("Hello & 'World'");</script>"#);
/// ```
///
/// Unescaping curly braces:
/// ```text
/// let input = "&#123;example&#125;";
/// let unescaped = unescape_chars(input, true);
/// assert_eq!(unescaped, "{example}");
/// ```
///
/// Unrecognized entities are preserved:
/// ```text
/// let input = "This is an &unknown; entity.";
/// let unescaped = unescape_chars(input, false);
/// assert_eq!(unescaped, "This is an &unknown; entity.");
/// ```
pub fn unescape_chars(input: &str, escape_braces: bool) -> String {
    if !input.contains('&') {
        return input.to_string();
    }
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            let mut entity = String::new();
            let mut has_semicolon = false;
            while let Some(&next_char) = chars.peek() {
                if next_char == ';' {
                    chars.next();
                    has_semicolon = true;
                    break;
                }
                entity.push(chars.next().unwrap());
            }
            match (entity.as_str(), has_semicolon) {
                ("amp", true) => result.push('&'),
                ("lt", true) => result.push('<'),
                ("gt", true) => result.push('>'),
                ("quot", true) => result.push('"'),
                ("#x27", true) => result.push('\''),
                ("#x2F", true) => result.push('/'),
                ("#123", true) if escape_braces => result.push('{'),
                ("#125", true) if escape_braces => result.push('}'),
                _ => {
                    result.push('&');
                    result.push_str(&entity);
                    if has_semicolon {
                        result.push(';');
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Recursively filter a Value with the function escape_chars
///
/// # Arguments
/// * `value` - A mutable reference to a JSON `Value`. It can be a string (`String`),
///             an object (`Object`), or an array (`Array`).
///
pub fn filter_value(value: &mut Value) {
    match value {
        Value::String(s) => *s = escape_chars(&unescape_chars(&s, true), true),
        Value::Object(obj) => for v in obj.values_mut() {
            filter_value(v) ;
        },
        Value::Array(arr) => for item in arr.iter_mut() {
            filter_value(item);
        },
        _ => {}
    }
}

/// Recursively filters the keys (names) of a Value with the function escape_chars
///
/// # Arguments
/// * `value` - A mutable reference to a JSON `Value`. It can be a string (`String`),
///             an object (`Object`), or an array (`Array`).
///
pub fn filter_value_keys(value: &mut Value) {
    match value {
        Value::Object(obj) => {
            let mut new_obj = serde_json::Map::new();

            for (key, val) in obj.iter_mut() {
                let new_key = escape_chars(&unescape_chars(key, true), true);
                filter_value_keys(val);
                new_obj.insert(new_key, val.clone());
            }

            *obj = new_obj;
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                filter_value_keys(item);
            }
        }
        _ => {}
    }
}
