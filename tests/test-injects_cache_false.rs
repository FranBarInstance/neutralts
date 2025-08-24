
// requires "preserve_order": serde_json = { version = "1.0", features = ["preserve_order"] }

// Here you can add things but you cannot modify or remove them.
const SCHEMA: &str = r#"{
    "config": {
        "infinite_loop_max_bifs": 555000,
        "comments": "remove",
        "cache_disable": false,
        "errors": "hide"
    },
    "inherit": {
        "snippets": {
            "__hello-nts": "<div>{:trans; ref:greeting-nts :}</div>",
            "inject": "{:;inject:}"
        },
        "declare": {
            "any": "*",
            "traversal": "/* \\\\* *\\.\\.*",
            "_test-nts": "en es fr de nts",
            "_test-nts-empty": "~ nts en es fr de",
            "_test-nts-asterisk": "*en* nts es fr de",
            "_test-nts-question": "en?nts nts es fr de",
            "_test-nts-dot": "en.nts es fr de"
        },
        "params": {},
        "locale": {
            "current": "en",
            "trans": {
                "en": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "en-US": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "en-UK": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "es": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "es-ES": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "de": {
                    "Hello nts": "Hallo",
                    "ref:greeting-nts": "Hallo"
                },
                "fr": {
                    "Hello nts": "Bonjour",
                    "ref:greeting-nts": "Bonjour"
                },
                "el": {
                    "Hello nts": "Γεια σας",
                    "ref:greeting-nts": "Γεια σας"
                }
            }
        }
    },
    "data": {
        "CONTEXT": {
            "GET": {
                "escape": "<>&\"'/{}"
            }
        },
        "__hello-nts": "Hello nts",
        "__ref-hello-nts": "__hello-nts",
        "__test-local": "local",
        "__test-nts": "nts",
        "__test-empty-nts": "",
        "__test-null-nts": null,
        "__test-zero-nts": 0,
        "__test-bool-true-string-nts": true,
        "__test-bool-true-num-nts": 1,
        "__test-bool-false-string-nts": false,
        "__test-bool-false-num-nts": 0,
        "__test-bool-false-empty-nts": "",
        "__test-arr-nts": [
            "one",
            "two",
            "three"
        ],
        "__test-arr-empty-nts": [],
        "__test-obj-empty-nts": {},
        "__test-obj-nts": {
            "level1": "Ok",
            "level1-obj": {
                "level1": "Ok",
                "level2-obj": {
                    "level2": "Ok",
                    "level3-arr": [
                        "one",
                        "two",
                        "three"
                    ]
                }
            }
        },
        "inject": "{:exit; 403 :}",
        "escape": "<>&\"'/{}",
        "double_escape": "&lt;&gt;&amp;&quot;&#x27;&#x2F;&#123;&#125;",
        "true": true,
        "false": false,
        "text": "text",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "text": "text",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    }
}"#;

#[test]
fn test_bif_inject_cache_false_neutral() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:neutral; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>{:neutral; {:;inject:} >> {:;inject:} :}</div>");
}

#[test]
fn test_bif_inject_cache_false_var() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:; {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_var_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:; foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_allow() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_allow_2() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:allow; _test-nts >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_bool() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_bool_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:bool; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}


#[test]
fn test_bif_inject_cache_false_coalesce() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:coalesce; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_code() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_code_scope() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:+code; {:;inject:} :}{:;inject:}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_code_nesting() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:code; {:;inject:} :} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_code_flags() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:flg; {:;inject:} :} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_code_flag_safe() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:flg; safe :} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:;inject:&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_contains() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:contains; /{:;inject:}/{:;inject:}/ >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_contains_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!contains; /{:;inject:}/{:;inject:}-foo/ >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_count_set() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:count; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_count_play() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:count; {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_data() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:data; {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_data_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:data; foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_date() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:date; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>{:;inject:}</div>");
}

#[test]
fn test_bif_inject_cache_false_defined() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:defined; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_defined_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!defined; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_each() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:each; __test-arr-nts k v >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;&#123;:exit; 403 :&#125;&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_each_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:each; {:;inject:} {:;inject:} {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_filled() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:filled; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_filled_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!filled; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_for() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n 1..3 >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;&#123;:exit; 403 :&#125;&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_for_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; {:;inject:} {:;inject:} {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_hash() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:hash; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>f7dfbc572362bb59401d33c504d01728</div>");
}

#[test]
fn test_bif_inject_cache_false_include() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_include_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:include; foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_include_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!include; foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_include_flags() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!include; {:flg; {:;inject:} :} >> foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_locale() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:locale; {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_locale_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:locale; foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_locale_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!locale; foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_locale_flags() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!locale; {:flg; {:;inject:} :} >> foo-{:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_moveto() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:moveto; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_param() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:code; {:param; {:;inject:} >> {:;inject:} :} {:param; {:;inject:} :} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_redirect() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:redirect; {:;inject:} >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_replace() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:replace; /{:;inject:}/{:;inject:}/ >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_same() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:same; /{:;inject:}/{:;inject:}/ >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_same_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!same; /{:;inject:}/{:;inject:}-foo/ >> {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_snippet() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:snippet; inject :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_snippet_eval() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:snippet; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_inject_cache_false_trans() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:trans; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div>&#123;:exit; 403 :&#125;</div>");
}

#[test]
fn test_bif_inject_cache_false_trans_negate() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:!trans; {:;inject:} :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}


#[test]
fn test_bif_inject_cache_false_join() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:join; /{:;inject:}/{:;inject:}/{:;inject:}/ :}</div>");
    let result = template.render();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}
