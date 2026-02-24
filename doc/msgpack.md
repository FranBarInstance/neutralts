# MessagePack Support

Neutral TS supports loading schema data from **MessagePack** (binary) in addition to standard JSON. The API is designed to be a symmetric counterpart to the existing JSON API.

## Methods

### `from_file_msgpack`

Constructs a new `Template` instance from a template file and MessagePack schema **bytes (in-memory)**. This is the binary counterpart to `from_file_value`.

```rust
use neutralts::Template;

let msgpack_bytes = get_bytes_from_somewhere();
let template = Template::from_file_msgpack("path/to/template.ntpl", &msgpack_bytes).unwrap();
let content = template.render();
```

### `merge_schema_msgpack`

Merges MessagePack schema **bytes (in-memory)** into the current template schema. This is the binary counterpart to `merge_schema_value`.

```rust
use neutralts::Template;

let mut template = Template::new().unwrap();
let msgpack_bytes = get_bytes_from_somewhere();
template.merge_schema_msgpack(&msgpack_bytes).unwrap();
```

### `merge_schema_msgpack_path`

Merges an external MessagePack schema **file path** into the current template schema. This is the binary counterpart to `merge_schema_path`.

```rust
use neutralts::Template;

let mut template = Template::new().unwrap();
template.merge_schema_msgpack_path("extra_data.msgpack").unwrap();
```

## Loading all from disk

If you want to load both the template and the MessagePack data from disk, follow the same strategy as with JSON:

```rust
use neutralts::Template;

// 1. Load template with empty data
let mut template = Template::from_file_msgpack("template.ntpl", &[]).unwrap();

// 2. Merge data from path
template.merge_schema_msgpack_path("data.msgpack").unwrap();
```

## Creating MessagePack files

You can create MessagePack files from Rust using the `rmp-serde` crate:

```rust
use serde_json::json;
use std::fs;

let schema = json!({
    "data": {
        "site_name": "My Optimized Site"
    }
});

let binary_data = rmp_serde::to_vec(&schema).unwrap();
fs::write("data.msgpack", binary_data).unwrap();
```

## Benefits

1. **Performance**: Binary deserialization is faster than parsing text-based JSON.
2. **Size**: Reduces disk space and memory footprint for large schema data.
3. **Symmetry**: The API follows the exact same pattern as the JSON API.
