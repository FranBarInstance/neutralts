
// tests requires "preserve_order": serde_json = { version = "1.0", features = ["preserve_order"] }

// Here you can add things but you cannot modify or remove them.
pub const SCHEMA: &str = r#"{
    "config": {
        "infinite_loop_max_bifs": 555000,
        "comments": "remove",
        "cache_disable": true,
        "disable_js": false,
        "errors": "hide"
    },
    "inherit": {
        "snippets": {
            "__hello-nts": "<div>{:trans; ref:greeting-nts :}</div>"
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

pub const SCHEMA_CACHE: &str = r#"{
    "config": {
        "infinite_loop_max_bifs": 555000,
        "comments": "remove",
        "errors": "hide",
        "comments": "remove",
        "cache_prefix": "neutral-cache",
        "cache_dir": "",
        "cache_on_post": false,
        "cache_on_get": true,
        "cache_on_cookies": true,
        "cache_disable": false,
        "disable_js": false,
        "filter_all": false
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
        "mailfotmated": "{::}",
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
