{:obj; ... :}
=============
**This feature is experimental.**

Executes an external script (Python or PHP via PHP-FPM) and processes its output. The script receives parameters and can access the template schema.

JSON obj file
```html
{:obj; fileobj.json :}
```

JSON obj inline:

```html
{:obj;
    {
        "engine": "python",
        "file": "script.py",
        "params": {},
        "callback": "main",
        "template": "template.ntpl"
    }
:}
```

JSON obj template inline:

```html
{:obj;
    {
        "engine": "python",
        "file": "script.py",
        "params": {},
        "callback": "main"
    }
    >>
    {:;local::varname:}
:}
```

The idea is to use a script that has its own template to assign values to the variables of that template. In a JSON file, an object with its properties, script to execute, template, etc., is defined.

Example Object:

```json
{
    "engine": "python",          // Optional, "python" (default) or "php"
    "file": "script.py",         // Required, path to script
    "schema": false,             // Optional, default false
    "schema_data": "__test-nts", // Optional, default none
    "venv": "/path/to/.env",     // Optional, default none
    "fpm": "unix:/run/php/php-fpm.sock", // Optional for PHP, default shown
    "params": {},                // Optional, parameters passed to the script
    "callback": "main",          // Optional, default "main"
    "template": "template.ntpl"  // Optional, template to process the result
}
```

The keys "file", "params", "venv", "fpm", "template" and "schema_data" accept variables `{:;varname:}`

Example Script:

```python
def main(params=None):
    schema = globals().get('__NEUTRAL_SCHEMA__')
    schema_data = globals().get('__NEUTRAL_SCHEMA_DATA__')
    return {
        "data": {
            "varname": "Hello from Python!"
        }
    }
```

`__NEUTRAL_SCHEMA__` (requires `"schema": true` in object) is read-only for accessing the schema. Access to `__NEUTRAL_SCHEMA__` can be slow, it is faster to use parameters.

`__NEUTRAL_SCHEMA_DATA__` (requires `"schema_data": "..."`) sends only one resolved value from schema data:

- Global data:
  - `"schema_data": "varname"`
  - `"schema_data": "varname->key->..."`
- Local data:
  - `"schema_data": "local::varname"`
  - `"schema_data": "local::varname->key->..."`

If the key does not exist, `__NEUTRAL_SCHEMA_DATA__` is `None`.

PHP script example (via PHP-FPM):

```php
<?php
function main($params = []) {
    $schema = $GLOBALS['__NEUTRAL_SCHEMA__'] ?? null;
    $schema_data = $GLOBALS['__NEUTRAL_SCHEMA_DATA__'] ?? null;
    return [
        "data" => [
            "varname" => "Hello from PHP!"
        ]
    ];
}
```

For PHP:

- `"engine": "php"`
- `"fpm"` can be:
  - `"unix:/run/php/php-fpm.sock"`
  - `"tcp://127.0.0.1:9000"`
  - `"127.0.0.1:9000"`
- If `"fpm"` is not set, default is `"unix:/run/php/php-fpm.sock"`.

It must return a dictionary where the variables are set in the format:

```text
{
    "data": {
        "varname": "value",
        "arrname": {
            "key": "value"
        }
    }
}
```

The variables are set in the template as "locals" `{:;local::varname:}`

Example for the previous script:

```html
{:obj; { "file": "script.py" } >>
    {:;local::varname:}
:}
```

Output:
```html
Hello from Python!
```

We can define a template in the object, in the latter case they are summed, first the template defined in the object will be shown and then the inline one.

Modifiers:
----------

```html
{:^obj; ... :}
{:+obj; ... :}
```

### Modifier: ^ (upline)

Removes previous whitespaces.

### Modifier: + (scope)

For more details about the "+" see "modifiers".

No flags
--------


Examples
--------

Basic usage with file:
```html
{:obj; objfile.json :}
```

Inline configuration with parameters:
```html
{:obj;
    {
        "file": "scripts/hello.py",
        "params": {
            "name": "World"
        }
    }
:}

{:obj;
    {
        "file": "scripts/hello.py",
        "params": {
            "name": "{:;varname:}"
        }
    }
:}
```

PHP-FPM usage:
```html
{:obj;
    {
        "engine": "php",
        "file": "scripts/hello.php",
        "fpm": "unix:/run/php/php-fpm.sock",
        "params": {
            "name": "World"
        }
    }
:}
```

Using `schema_data`:
```html
{:obj;
    {
        "file": "scripts/data.py",
        "schema_data": "__test-arr-nts"
    }
:}

{:obj;
    {
        "file": "scripts/data.py",
        "schema_data": "__test-obj-nts->level1-obj"
    }
:}

{:obj;
    {
        "file": "scripts/data.py",
        "schema_data": "local::array->text"
    }
:}
```

Nested path examples:
```html
{:obj;
    {
        "file": "scripts/data.py",
        "schema_data": "__test-obj-nts->level1-obj->level2-obj->level2"
    }
:}

{:obj;
    {
        "file": "scripts/data.py",
        "schema_data": "local::nested-obj->Lorem->Ipsum->Dolor->Sit->Amet"
    }
:}
```

Using template with script output:
```html
{:obj;
    {
        "file": "scripts/data.py",
        "template": "templates/view.ntpl"
    }
:}
```

Limitations
------------

Python is slow and executing Python as a subprocess is even slower, use "{:cache;" when possible.

PHP via FPM adds network/socket overhead per call, use "{:cache;" when possible.

It is not the same to use "obj" to replace multiple variables than to, for example, create a complete form, in the first case performance will be affected until it is unacceptable, in the second case the loss is likely not noticeable.

---
