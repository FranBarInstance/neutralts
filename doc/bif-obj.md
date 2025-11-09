{:obj; ... :}
=============
**This feature is experimental.**

Executes a external script (currently only Python) and processes its output. The script receives parameters and can access the template schema.

JSON obj file
```html
{:obj; fileobj.json :}
```

JSON obj inline:

```html
{:obj;
    {
        "engine": "Python",
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
        "engine": "Python",
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
    "engine": "Python",          // Optional, default "Python"
    "file": "script.py",         // Required, path to script
    "schema": false,             // Optional, default false
    "venv": "/path/to/.env",     // Optional, default none
    "params": {},                // Optional, parameters passed to the script
    "callback": "main",          // Optional, default "main"
    "template": "template.ntpl"  // Optional, template to process the result
}
```

The keys "file", "params", "venv" and "template" accept variables `{:;varname:}`

Example Script:

```python
def main(params=None):
    schema = globals().get('__NEUTRAL_SCHEMA__')
    return {
        "data": {
            "varname": "Hello from Python!"
        }
    }
```

`__NEUTRAL_SCHEMA__` (requires `"schema": true` in object) is read-only for accessing the schema. Access to `__NEUTRAL_SCHEMA__` can be slow, it is faster to use parameters.

It must return a dictionary where the variables are set in the format:

```text
{
    "data": {
        "varname": "value",
        "arrname: {
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

It is not the same to use "obj" to replace multiple variables than to, for example, create a complete form, in the first case performance will be affected until it is unacceptable, in the second case the loss is likely not noticeable.

---
