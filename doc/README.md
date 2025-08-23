![neutral](https://gitlab.com/neutralfw/neutralts/-/raw/master/top-neutralts.png)

Rust Web Template Engine
========================

Neutral is a **template engine** for the Web, an alternative to *handlebars*, designed to work with **any programming language** (language-agnostic) via IPC/Package and natively as library/crate in Rust.

In this simple PWA example, all three use exactly the same templates.

- [Rust PWA example](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/rust)
- [Python PWA example IPC](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/python)
- [Python PWA example Package](https://github.com/FranBar1966/neutraltemplate/tree/master/examples)
- [PHP PWA example](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/php)
- [Template](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app/neutral)

(*) For non-Rust requires an IPC server that you can download from the [IPC repository](https://gitlab.com/neutralfw/ipc) - [IPC server](https://gitlab.com/neutralfw/ipc/-/releases). Alternatively in Python you can use [PYPI Package](https://pypi.org/project/neutraltemplate/)

The documentation of the **web template** engine is here: [template engine doc](https://docs.rs/neutralts/latest/neutralts/doc/) and **Rust** documentation here: [rust doc](https://docs.rs/neutralts/latest/neutralts/).

Rust
----

In Rust it is enough with two methods, create the template with a file and a schema and then render:

```text
// Data
let schema = json!({
    "data": {
        "hello": "Hello, World!",
        "site": {
            "name": "My Site"
        }
    }
});

// Create template
// In file.ntpl use {:;hello:} and {:;site->name:} for show data.
let template = Template::from_file_value("file.ntpl", schema).unwrap();

// Render template
let content = template.render();
```

Links
-----

- [Rust docs](https://docs.rs/neutralts/latest/neutralts/)
- [Template docs](https://docs.rs/neutralts/latest/neutralts/doc/)
- [IPC server](https://gitlab.com/neutralfw/ipc/-/releases)
- [IPC server and clients](https://gitlab.com/neutralfw/ipc)
- [Repository](https://gitlab.com/neutralfw/neutralts)
- [Crate](https://crates.io/crates/neutralts)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Example Web App](https://gitlab.com/neutralfw/neutralts/-/tree/master/web-app)
- [Examples](https://gitlab.com/neutralfw/neutralts/-/tree/master/examples)

------
