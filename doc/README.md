![neutral](https://gitlab.com/neutralfw/neutralts/-/raw/master/top-neutralts.png)

Rust Web Template Engine
========================

Neutral TS is a **safe, modular, language-agnostic template engine** built in Rust. It works as a **native Rust library** or via **IPC** for other languages like Python and PHP. With Neutral TS you can reuse the **same template across multiple languages** with consistent results.

Examples for [Rust](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/rust), [Python](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python), [PHP](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/php), [Node.js](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/node) and [Go](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/go) here: [download](https://github.com/FranBarInstance/neutralts-docs/releases). All PWA [examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples) use the same template: [Neutral templates](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/neutral).

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
- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [IPC server/clients](https://github.com/FranBarInstance/neutral-ipc/)
- [Repository](https://github.com/FranBarInstance/neutralts)
- [Examples/Docs](https://github.com/FranBarInstance/neutralts-docs/)

------
