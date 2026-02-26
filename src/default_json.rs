pub const DEFAULT: &str = r#"{
    "_comment_:License": "License in the terms described in the LICENSE file.",
    "version": "0.0.0",
    "config": {
        "infinite_loop_max_bifs": 555000,
        "_comment_:comments": "keep|remove, default or empty keep",
        "comments": "remove",
        "_comment_:errors": "show|hide, default or empty show",
        "error": {
            "show": true
        },
        "app": {},
        "cache_prefix": "neutral-cache",
        "cache_dir": "",
        "cache_on_post": false,
        "cache_on_get": true,
        "cache_on_cookies": true,
        "cache_disable": false,
        "filter_all": false,
        "disable_js": false,
        "debug_expire": 3600,
        "debug_file": ""
    },
    "data": {
        "CONTEXT": {
            "CONFIG": {},
            "ROUTE": "",
            "HOST": "",
            "GET": {},
            "POST": {},
            "HEADERS": {},
            "REQUEST": {},
            "FILES": {},
            "COOKIES": {},
            "SESSION": {},
            "ENV": {}
        },
        "__hello-nts": "Hello nts",
        "__ref-hello-nts": "__hello-nts",
        "__test-nts": "nts",
        "__test-arr_nts": [
            "one",
            "two",
            "three"
        ],
        "__test-obj_nts": {
            "level1": "Ok",
            "level1_arr": {
                "level2": "Ok",
                "level2_obj": {
                    "level3": "Ok",
                    "level3_arr": [
                        "one",
                        "two",
                        "three"
                    ]
                }
            }
        }
    },
    "inherit": {
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
        },
        "snippets": {
            "__hello-nts": "<div>{:trans; ref:greeting-nts :}</div>"
        },
        "snippets_set_dir": {},
        "declare": {
            "any": "*",
            "traversal": "/* \\\\* *\\.\\.*"
        },
        "params": {}
    },
    "__moveto": {},
    "__indir": {},
    "__error": []
}"#;
