
use neutralts::constants::*;
use neutralts::utils::*;
use serde_json::Value;
use serde_json::json;

const HTML_SOURCE: &str = r#"<!DOCTYPE html>
<html lang="{:lang;:}">
    <head>
        {:*
            comment
        *:}
        <title>{:trans; Site title :}</title>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        {:snippet; current-theme:head :}
        <link rel="stylesheet" href="bootstrap.min.css">
    </head>
    <body class="{:;body-class:}">
        {:snippet; current-theme:body_begin  :}
        {:snippet; current-theme:body-content :}
        {:snippet; current-theme:body-footer  :}
        <script src="jquery.min.js"></script>
    </body>
</html>"#;

#[test]
fn test_extract_blocks_from_html() {
    let expected = vec![
        (28, 37),   // {:lang;:}
        (59, 94),   // {:* comment *:}
        (110, 132), // {:trans; Site title :}
        (257, 289), // {:snippet; current-theme:head :}
        (376, 391), // {:;body-class:}
        (402, 441), // {:snippet; current-theme:body_begin  :}
        (450, 490), // {:snippet; current-theme:body-content :}
        (499, 539), // {:snippet; current-theme:body-footer  :}
    ];
    assert_eq!(extract_blocks(HTML_SOURCE).unwrap(), expected);
}

#[test]
fn test_merge_schema() {
    let mut a: Value = serde_json::json!({
        "name": "John",
        "age": 30,
        "address": {
            "city": "New York"
        }
    });

    let b: Value = serde_json::json!({
        "age": 25,
        "email": "john@example.com",
        "address": {
            "street": "123 Main St"
        }
    });

    merge_schema(&mut a, &b);

    let expected: Value = serde_json::json!({
        "name": "John",
        "age": 25,
        "email": "john@example.com",
        "address": {
            "city": "New York",
            "street": "123 Main St"
        }
    });

    assert_eq!(a, expected);
}

#[test]
fn test_merge_schema_nested() {
    let mut a: Value = serde_json::json!({
        "user": {
            "name": "John",
            "details": {
                "age": 30,
                "address": {
                    "city": "New York"
                }
            }
        }
    });

    let b: Value = serde_json::json!({
        "user": {
            "details": {
                "age": 25,
                "address": {
                    "street": "123 Main St"
                }
            }
        }
    });

    merge_schema(&mut a, &b);

    let expected: Value = serde_json::json!({
        "user": {
            "name": "John",
            "details": {
                "age": 25,
                "address": {
                    "city": "New York",
                    "street": "123 Main St"
                }
            }
        }
    });

    assert_eq!(a, expected);
}

#[test]
fn test_merge_schema_non_object() {
    let mut a: Value = serde_json::json!(42);
    let b: Value = serde_json::json!("hello");

    merge_schema(&mut a, &b);

    let expected: Value = serde_json::json!("hello");

    assert_eq!(a, expected);
}

#[test]
fn test_strip_prefix_suffix() {
    let str = "{:defined; name >> hello :}";
    let prefix = BIF_OPEN;
    let suffix = BIF_CLOSE;

    let expected = "defined; name >> hello ";
    assert_eq!(strip_prefix_suffix(str, prefix, suffix), expected);
}

#[test]
fn test_get_from_key() {
    let schema = json!({
        "name": "John",
        "age": 30,
    });

    assert_eq!(get_from_key(&schema, "name"), "John");
    assert_eq!(get_from_key(&schema, "age"), "30");
}

#[test]
fn test_is_empty_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert!(!is_empty_key(&schema, "true"));
    assert!(!is_empty_key(&schema, "false"));
    assert!(!is_empty_key(&schema, "hello"));
    assert!(!is_empty_key(&schema, "zero"));
    assert!(!is_empty_key(&schema, "one"));
    assert!(!is_empty_key(&schema, "spaces"));
    assert!(is_empty_key(&schema, "empty"));
    assert!(is_empty_key(&schema, "null"));
    assert!(is_empty_key(&schema, "emptyarr"));
    assert!(!is_empty_key(&schema, "array/true"));
    assert!(!is_empty_key(&schema, "array/false"));
    assert!(!is_empty_key(&schema, "array/hello"));
    assert!(!is_empty_key(&schema, "array/zero"));
    assert!(!is_empty_key(&schema, "array/one"));
    assert!(!is_empty_key(&schema, "array/spaces"));
    assert!(is_empty_key(&schema, "array/empty"));
    assert!(is_empty_key(&schema, "array/null"));
    assert!(is_empty_key(&schema, "non_existent_key"));
}


#[test]
fn test_is_bool_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert!(is_bool_key(&schema, "true"));
    assert!(!is_bool_key(&schema, "false"));
    assert!(is_bool_key(&schema, "hello"));
    assert!(!is_bool_key(&schema, "zero"));
    assert!(is_bool_key(&schema, "one"));
    assert!(is_bool_key(&schema, "spaces"));
    assert!(!is_bool_key(&schema, "empty"));
    assert!(!is_bool_key(&schema, "null"));
    assert!(!is_bool_key(&schema, "emptyarr"));
    assert!(is_bool_key(&schema, "array/true"));
    assert!(!is_bool_key(&schema, "array/false"));
    assert!(is_bool_key(&schema, "array/hello"));
    assert!(!is_bool_key(&schema, "array/zero"));
    assert!(is_bool_key(&schema, "array/one"));
    assert!(is_bool_key(&schema, "array/spaces"));
    assert!(!is_bool_key(&schema, "array/empty"));
    assert!(!is_bool_key(&schema, "array/null"));
    assert!(!is_bool_key(&schema, "non_existent_key"));
}

#[test]
fn test_is_array_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert!(!is_array_key(&schema, "true"));
    assert!(!is_array_key(&schema, "false"));
    assert!(!is_array_key(&schema, "hello"));
    assert!(!is_array_key(&schema, "zero"));
    assert!(!is_array_key(&schema, "one"));
    assert!(!is_array_key(&schema, "spaces"));
    assert!(!is_array_key(&schema, "empty"));
    assert!(!is_array_key(&schema, "null"));
    assert!(is_array_key(&schema, "emptyarr"));
    assert!(!is_array_key(&schema, "array/true"));
    assert!(!is_array_key(&schema, "array/false"));
    assert!(!is_array_key(&schema, "array/hello"));
    assert!(!is_array_key(&schema, "array/zero"));
    assert!(!is_array_key(&schema, "array/one"));
    assert!(!is_array_key(&schema, "array/spaces"));
    assert!(!is_array_key(&schema, "array/empty"));
    assert!(!is_array_key(&schema, "array/null"));
    assert!(!is_array_key(&schema, "non_existent_key"));
}

#[test]
fn test_is_defined_key() {
    let schema = json!({
        "true": true,
        "false": false,
        "hello": "hello",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "hello": "hello",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    });

    assert!(is_defined_key(&schema, "true"));
    assert!(is_defined_key(&schema, "false"));
    assert!(is_defined_key(&schema, "hello"));
    assert!(is_defined_key(&schema, "zero"));
    assert!(is_defined_key(&schema, "one"));
    assert!(is_defined_key(&schema, "spaces"));
    assert!(is_defined_key(&schema, "empty"));
    assert!(!is_defined_key(&schema, "null"));
    assert!(is_defined_key(&schema, "emptyarr"));
    assert!(is_defined_key(&schema, "array/true"));
    assert!(is_defined_key(&schema, "array/false"));
    assert!(is_defined_key(&schema, "array/hello"));
    assert!(is_defined_key(&schema, "array/zero"));
    assert!(is_defined_key(&schema, "array/one"));
    assert!(is_defined_key(&schema, "array/spaces"));
    assert!(is_defined_key(&schema, "array/empty"));
    assert!(!is_defined_key(&schema, "array/null"));
    assert!(!is_defined_key(&schema, "non_existent_key"));
}

#[test]
fn test_get_code_position() {
    let src = r#"!snippet; {:defined; name >> snippet_name :}{:else: none :} >> <div>... {:* comment *:} ...</div> "#;
    assert_eq!(get_code_position(src), Some(60));
}

#[test]
fn test_wildcard_match() {
    // Basic match
    assert!(wildcard_match("hello", "hello"));

    // Wildcard '*' matches any sequence of characters
    assert!(wildcard_match("hello", "*"));
    assert!(wildcard_match("hello", "h*o"));
    assert!(wildcard_match("hello", "he*llo"));
    assert!(wildcard_match("hello", "hell*"));
    assert!(wildcard_match("hello", "*hello"));

    // Wildcard '?' matches any single character
    assert!(wildcard_match("hello", "h?llo"));
    assert!(wildcard_match("hello", "?ello"));
    assert!(wildcard_match("hello", "he?lo"));
    assert!(wildcard_match("hello", "hell?"));
    assert!(wildcard_match("hello", "*ell?"));

    // Mixed usage of '*' and '?'
    assert!(wildcard_match("hello", "h?*o"));
    assert!(wildcard_match("hello", "h*ll?"));
    assert!(wildcard_match("hello", "?*llo"));
    assert!(wildcard_match("hello", "he*l?"));

    // Escaping special characters
    assert!(wildcard_match("hell*o", "hell\\*o"));
    assert!(wildcard_match("hell.o", "hell\\.o"));
    assert!(wildcard_match("hell?o", "hell\\?o"));

    // Empty pattern
    assert!(wildcard_match("", ""));

    // Special character '~' matches empty string
    assert!(wildcard_match("", "~"));
}


#[test]
fn test_find_tag_position() {
    // Basic match for opening tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<html"), Some(39));

    // Match for closing tag
    assert_eq!(find_tag_position(HTML_SOURCE, "</html"), Some(598));

    // Nested tags
    assert_eq!(find_tag_position(HTML_SOURCE, "<head"), Some(50));
    assert_eq!(find_tag_position(HTML_SOURCE, "</head"), Some(351));

    // Match for meta tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<meta"), Some(171));

    // Non-existent tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<nonexistent>"), None);

    // Self-closing tag
    assert_eq!(find_tag_position(HTML_SOURCE, "<link"), Some(346));

    // Empty string source
    assert_eq!(find_tag_position("", "<html"), None);
}

#[test]
fn test_escape_chars_no_special_chars_false() {
    let input = "Hello, World!";
    let result = escape_chars(input, false);
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_escape_chars_no_special_chars_true() {
    let input = "Hello, World!";
    let result = escape_chars(input, true);
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_escape_chars_with_special_chars_false() {
    let input = "&<>'\"/{}";
    let result = escape_chars(input, false);
    assert_eq!(result, "&amp;&lt;&gt;&#x27;&quot;&#x2F;{}");
}

#[test]
fn test_escape_chars_with_special_chars_true() {
    let input = "&<>'\"/{}";
    let result = escape_chars(input, true);
    assert_eq!(result, "&amp;&lt;&gt;&#x27;&quot;&#x2F;&#123;&#125;");
}

#[test]
fn test_escape_chars_with_braces_false() {
    let input = "{}";
    let result = escape_chars(input, false);
    assert_eq!(result, "{}");
}

#[test]
fn test_escape_chars_with_braces_true() {
    let input = "{}";
    let result = escape_chars(input, true);
    assert_eq!(result, "&#123;&#125;");
}

#[test]
fn test_escape_chars_mixed_ascii_and_non_ascii_false() {
    let input = "Hello, 🌍! &<>'\"/";
    let result = escape_chars(input, false);
    assert_eq!(result, "Hello, 🌍! &amp;&lt;&gt;&#x27;&quot;&#x2F;");
}

#[test]
fn test_escape_chars_mixed_ascii_and_non_ascii_true() {
    let input = "Hello, 🌍! &<>'\"/";
    let result = escape_chars(input, true);
    assert_eq!(result, "Hello, 🌍! &amp;&lt;&gt;&#x27;&quot;&#x2F;");
}

#[test]
fn test_escape_chars_empty_string_false() {
    let input = "";
    let result = escape_chars(input, false);
    assert_eq!(result, "");
}

#[test]
fn test_escape_chars_empty_string_true() {
    let input = "";
    let result = escape_chars(input, true);
    assert_eq!(result, "");
}

#[test]
fn test_escape_chars_only_braces_false() {
    let input = "{}";
    let result = escape_chars(input, false);
    assert_eq!(result, "{}");
}

#[test]
fn test_escape_chars_only_braces_true() {
    let input = "{}";
    let result = escape_chars(input, true);
    assert_eq!(result, "&#123;&#125;");
}

#[test]
fn test_escape_chars_all_braces_escaped_false() {
    let input = "{{}}";
    let result = escape_chars(input, false);
    assert_eq!(result, "{{}}");
}

#[test]
fn test_escape_chars_all_braces_escaped_true() {
    let input = "{{}}";
    let result = escape_chars(input, true);
    assert_eq!(result, "&#123;&#123;&#125;&#125;");
}

#[test]
fn test_escape_chars_with_other_ascii_chars_false() {
    let input = "abc123!@#$%^&*()";
    let result = escape_chars(input, false);
    assert_eq!(result, "abc123!@#$%^&amp;*()");
}

#[test]
fn test_escape_chars_with_other_ascii_chars_true() {
    let input = "abc123!@#$%^&*()";
    let result = escape_chars(input, true);
    assert_eq!(result, "abc123!@#$%^&amp;*()");
}

#[test]
fn test_unescape_chars_mixed_ascii_and_non_ascii_false() {
    let input = "Hello, 🌍! &amp;&lt;&gt;&#x27;&quot;&#x2F;&#123;&#125;";
    let result = unescape_chars(input, false);
    assert_eq!(result, "Hello, 🌍! &<>'\"/&#123;&#125;");
}

#[test]
fn test_unescape_chars_mixed_ascii_and_non_ascii_true() {
    let input = "Hello, 🌍! &amp;&lt;&gt;&#x27;&quot;&#x2F;&#123;&#125;";
    let result = unescape_chars(input, true);
    assert_eq!(result, "Hello, 🌍! &<>'\"/{}");
}

#[test]
fn test_unescape_chars_no_entities() {
    let input = "Hello, World!";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "Hello, World!");
    assert_eq!(result_true, "Hello, World!");
}

#[test]
fn test_unescape_chars_with_entities() {
    let input = "&amp;<>&quot;&#x27;&#x2F;";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "&<>\"'/");
    assert_eq!(result_true, "&<>\"'/");
}

#[test]
fn test_unescape_chars_with_braces_false() {
    let input = "&#123;&#125;";
    let result = unescape_chars(input, false);
    assert_eq!(result, "&#123;&#125;");
}

#[test]
fn test_unescape_chars_with_braces_true() {
    let input = "&#123;&#125;";
    let result = unescape_chars(input, true);
    assert_eq!(result, "{}");
}

#[test]
fn test_unescape_chars_mixed_ascii_and_non_ascii() {
    let input = "Hello, 🌍! &amp;<>&quot;&#x27;&#x2F;";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "Hello, 🌍! &<>\"'/");
    assert_eq!(result_true, "Hello, 🌍! &<>\"'/");
}

#[test]
fn test_unescape_chars_empty_string() {
    let input = "";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "");
    assert_eq!(result_true, "");
}

#[test]
fn test_unescape_chars_only_braces_false() {
    let input = "{}";
    let result = unescape_chars(input, false);
    assert_eq!(result, "{}");
}

#[test]
fn test_unescape_chars_only_braces_true() {
    let input = "{}";
    let result = unescape_chars(input, true);
    assert_eq!(result, "{}");
}

#[test]
fn test_unescape_chars_all_braces_escaped_false() {
    let input = "&#123;&#123;&#125;&#125;";
    let result = unescape_chars(input, false);
    assert_eq!(result, "&#123;&#123;&#125;&#125;");
}

#[test]
fn test_unescape_chars_all_braces_escaped_true() {
    let input = "&#123;&#123;&#125;&#125;";
    let result = unescape_chars(input, true);
    assert_eq!(result, "{{}}");
}

#[test]
fn test_unescape_chars_invalid_entity() {
    let input = "&invalid;";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "&invalid;");
    assert_eq!(result_true, "&invalid;");
}

#[test]
fn test_unescape_chars_partial_entity() {
    let input = "&amp";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "&amp");
    assert_eq!(result_true, "&amp");
}

#[test]
fn test_unescape_chars_malformed_entity() {
    let input = "&amp;";
    let result_false = unescape_chars(input, false);
    let result_true = unescape_chars(input, true);
    assert_eq!(result_false, "&");
    assert_eq!(result_true, "&");
}

#[test]
fn test_filter_value_nested() {
    let mut input = json!({
        "data": {
            "var1": "<div>{:;:}",
            "var2": 30
        }
    });
    let expected = json!({
        "data": {
            "var1": "&lt;div&gt;&#123;:;:&#125;",
            "var2": 30
        }
    });

    filter_value(&mut input);
    assert_eq!(input, expected);
}

#[test]
fn test_filter_name() {
    // JSON de entrada
    let mut input = json!({
        "{:name:}": "John",
        "<age>": 30,
        "<{address}>": {
            "{city}": "New York",
            "<zip>": "10001"
        }
    });

    let expected = json!({
        "&#123;:name:&#125;": "John",
        "&lt;age&gt;": 30,
        "&lt;&#123;address&#125;&gt;": {
            "&#123;city&#125;": "New York",
            "&lt;zip&gt;": "10001"
        }
    });

    filter_value_keys(&mut input);
    assert_eq!(input, expected);
}
