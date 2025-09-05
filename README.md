![neutral](https://raw.githubusercontent.com/FranBarInstance/neutralts/refs/heads/master/top-neutralts.png)

Web Template Engine - Neutral TS
================================

Neutral TS is a **safe, modular, language-agnostic template engine** built in Rust.
It works as a **native Rust library** or via **IPC** for other languages like Python and PHP.
With Neutral TS you can reuse the **same template across multiple languages** with consistent results.

For non-Rust requires an IPC server that you can download from the [IPC repository](https://github.com/FranBarInstance/neutral-ipc) - [IPC server](https://github.com/FranBarInstance/neutral-ipc/releases). Alternatively in Python you can use [PYPI Package](https://pypi.org/project/neutraltemplate/)

The documentation of the **web template** engine is here: [template engine doc](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/) and **Rust** documentation here: [rust doc](https://docs.rs/neutralts/latest/neutralts/).

Template Engine - Features
--------------------------

It allows you to create templates compatible with any system and any programming language.

* Safe
* Language-agnostic
* Modular
* Parameterizable
* Efficient
* Inheritance
* Cache modular and !cache
* Objects
* JS fetch
* Parse files
* Embed files
* Localization
* Loops: for and each
* Snippets
* Nesting, grouping and wrapping
* Redirections: HTTP y JavaScript
* Exit with error: 403, 404, 503, ...
* Comments

How it works
------------

Neutral TS template engine offers two main ways to integrate with other programming languages:

* In Rust: You can use Neutral TS template engine as a library by downloading the crate.

* In other programming languages: Inter-Process Communication ([IPC](https://gitlab.com/neutralfw/ipc)) is necessary, similar to how databases like MariaDB work.

Imagine a database. It has a server, and different programming languages access the data through a client. This means that if you run a "SELECT ..." query from any programming language, the result will always be the same.

Similarly, Neutral TS has an IPC server, and each programming language has a client. No matter where you run the template, the result will always be the same.

Thanks to this, and to its modular and parameterizable design, it is possible to create utilities or plugins that will work everywhere. For example, you can develop tools to create forms or form fields and create your own libraries of "snippets" for repetitive tasks.

Localization
------------

Neutral TS template engine provides powerful and easy-to-use translation utilities... define the translation in a JSON:

```json
"locale": {
    "current": "en",
    "trans": {
        "en": {
            "Hello": "Hello",
            "ref:greeting-nts": "Hello"
        },
        "es": {
            "Hello": "Hola",
            "ref:greeting-nts": "Hola"
        },
        "de": {
            "Hello": "Hallo",
            "ref:greeting-nts": "Hallo"
        },
        "fr": {
            "Hello": "Bonjour",
            "ref:greeting-nts": "Bonjour"
        },
        "el": {
            "Hello": "Γεια σας",
            "ref:greeting-nts": "Γεια σας"
        }
    }
}
```

Now you can use:

```neutral
{:trans; Hello :}
```

Actually you can always use "trans" because if there is no translation it returns the text.  See: [locale](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/#locale--) and [trans](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/#trans--).

Bif layout (Build-in function)
------------------------------

```neutral

    .-- open bif
    |    .-- bif name
    |    |   .-- name separator
    |    |   |     .-- params
    |    |   |     |    .-- params/code separator
    |    |   |     |    |    .-- code
    |    |   |     |    |    |   .-- close bif
    |    |   |     |    |    |   |
    v    v   v     v    v    v   v
    -- ----- - -------- -- ----- --
    {:snippet; snipname >>  ...  :}
    ------------------------------
            ^ -------------------
            |         ^
            |         |
            |         `-- source
            `-- Build-in function

```

Bif example: (See: [syntax](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/#syntax))

```neutral
{:filled; varname >>
    Hello!
:}
```

Neutral TS template engine is based on Bifs with block structure, we call the set of nested Bifs of the same level a block:

```neutral

              .-- {:coalesce;
              |       {:code;
              |           {:code; ... :}
              |           {:code; ... :}
    Block --> |           {:code; ... :}
              |       :}
              |       {:code;
              |           {:code; ... :}
              |       :}
              `-- :}

                  {:coalesce;
              .------ {:code;
              |           {:code; ... :}
    Block --> |           {:code; ... :}
              |           {:code; ... :}
              `------ :}
              .------ {:code;
    Block --> |           {:code; ... :}
              `------ :}
                  :}

```

Short circuit at block level, if varname is not defined, the following ">>" is not evaluated:

```neutral
{:defined; varname >>
    {:code;
        {:code;
            ...
        :}
    :}
:}
```

By design all Bifs can be nested and there can be a Bif anywhere in another Bif except in the name.

Data
----

The data is defined in a JSON:

```json
"data": {
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
}
```

And they are displayed with the bif {:; ... :} (var)

Simple variable:

```neutral
{:;hello:}
```

Arrays with the "->" operator

```neutral
{:;array->hello:}
```

Snippets
--------

Snippet is a tool that can be used in a similar way to a function, it defines a snippet:

```neutral
{:snippet; name >>
    Any content here, including other snippet.
:}
```

From then on you can invoke it like this:

```neutral
{:snippet; name :}
```

See: [snippet](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/#snippet--).

Cache
-----

The cache is modular, allowing only parts of the template to be included in the cache:

```plaintext
<!DOCTYPE html>
<html>
    <head>
        <title>Template engine cache</title>
    </head>
    <body>
        {:cache; /120/ >>
            <div>{:code; ... :}</div>
        :}
        <div>{:date; %H:%M:%S :}</div>
        {:cache; /120/ >>
            <div>{:code; ... :}</div>
        :}
    </body>
</html>
```
Or exclude parts of the cache, the previous example would be much better like this:

```plaintext
{:cache; /120/ >>
    <!DOCTYPE html>
    <html>
        <head>
            <title>Template engine cache</title>
        </head>
        <body>
            <div>{:code; ... :}</div>
            {:!cache;
                {:date; %H:%M:%S :}
            :}
            <div>{:code; ... :}</div>
        </body>
    </html>
:}
```

Fetch
-----

Neutral TS template engine provides a basic JavaScript to perform simple `fetch` requests:

```plaintext
<!DOCTYPE html>
<html>
    <head>
        <title>Template engine</title>
    </head>
    <body>
        {:fetch; "/form-login" >>
            <div>Loading...</div>
        :}
    </body>
</html>
```
See: [fetch](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/#fetch--).

Object
------

`obj` allows you to execute scripts in other languages like Python

```html
{:obj;
    {
        "engine": "Python",
        "file": "script.py",
        "template": "template.ntpl"
    }
:}
```
See: [obj](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/#obj--).

Web template - example
----------------------

```html
{:*
    comment
*:}
{:locale; locale.json :}
{:include; theme-snippets.ntpl :}
<!DOCTYPE html>
<html lang="{:lang;:}">
    <head>
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
</html>
```

Usage
-----

You need two things, a template file and a json schema:

```plaintext
{
    "config": {
        "comments": "remove",
        "cache_prefix": "neutral-cache",
        "cache_dir": "",
        "cache_on_post": false,
        "cache_on_get": true,
        "cache_on_cookies": true,
        "cache_disable": false,
        "filter_all": false,
        "disable_js": false
    },
    "inherit": {
        "locale": {
            "current": "en",
            "trans": {
                "en": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "es": {
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
            "ROUTE": "",
            "HOST": "",
            "GET": {},
            "POST": {},
            "HEADERS": {},
            "FILES": {},
            "COOKIES": {},
            "SESSION": {},
            "ENV": {}
        },
        "site_name": "MySite",
        "site": {
            "name": "MySite",
        }
    }
}
```

Template file.ntpl:

```text
{:;site_name:}
```

Or for array:

```text
{:;site->name:}
```

Native use (Rust)
-----------------

```text
use neutralts::Template;
use serde_json::json;

let template = Template::from_file_value("file.ntpl", schema).unwrap();
let content = template.render();

// e.g.: 200
let status_code = template.get_status_code();

// e.g.: OK
let status_text = template.get_status_text();

// empty if no error
let status_param = template.get_status_param();

// act accordingly at this point according to your framework
```

### Rust examples

 - [PWA example](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/rust)
 - [actix-web](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples/actix)
 - [warp](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples/warp)
 - [axum](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples/actix)
 - [rocket](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples/rocket)
 - [examples](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples)

Python - Package
----------------

```text
pip install neutraltemplate
```

```text
from neutraltemplate import NeutralTemplate

template = NeutralTemplate("file.ntpl", schema)
contents = template.render()

# e.g.: 200
status_code = template.get_status_code()

# e.g.: OK
status_text = template.get_status_text()

# empty if no error
status_param = template.get_status_param()

# act accordingly at this point according to your framework
```

Python - IPC
------------

Requires an IPC server that you can download from the [repository](https://github.com/FranBarInstance/neutral-ipc), and an IPC client that you can download here: [IPC client](https://gitlab.com/neutralfw/ipc/-/tree/master/python)

```text
from NeutralIpcTemplate import NeutralIpcTemplate

template = NeutralIpcTemplate("file.ntpl", schema)
contents = template.render()

# e.g.: 200
status_code = template.get_status_code()

# e.g.: OK
status_text = template.get_status_text()

# empty if no error
status_param = template.get_status_param()

# act accordingly at this point according to your framework
```

### Python examples

- [PWA example IPC](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/python)
- [PWA example Package](https://github.com/FranBarInstance/neutraltemplate/tree/master/examples)
- [Simple example](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples/python)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [IPC client](https://gitlab.com/neutralfw/ipc/-/tree/master/python)
- [IPC server](https://github.com/FranBarInstance/neutral-ipc)

PHP
---

Requires an IPC server that you can download from the [repository](https://github.com/FranBarInstance/neutral-ipc), and an IPC client that you can download here: [IPC client](https://gitlab.com/neutralfw/ipc/-/tree/master/php)

```text
include 'NeutralIpcTemplate.php';

$template = new NeutralIpcTemplate("file.ntpl", $schema);
$contents = $template->render();

// e.g.: 200
$status_code = $template->get_status_code();

// e.g.: OK
$status_text = $template->get_status_text();

// empty if no error
$status_param = $template->get_status_param();

// act accordingly at this point according to your framework
```

### PHP examples

- [PWA example](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/php)
- [example](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples/php)
- [IPC client](https://gitlab.com/neutralfw/ipc/-/tree/master/php)
- [IPC server](https://gitlab.com/neutralfw/ipc)

Links
-----

Neutral TS template engine.

- [Rust docs](https://docs.rs/neutralts/latest/neutralts/)
- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [IPC server](https://github.com/FranBarInstance/neutral-ipc/releases)
- [IPC server and clients](https://github.com/FranBarInstance/neutral-ipc)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Example Web App](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app)
- [Examples](https://franbarinstance.github.io/neutralts-docs/)
