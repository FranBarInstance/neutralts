{:obj; ... :}
=============

Executes a external script (currently only Python) and processes its output. The script receives parameters and can access the template schema.

```html
{:obj; fileobj.json :}
```

The idea is to use a script that has its own template to assign values to the variables of that template. In a JSON file, an object with its properties, script to execute, template, etc., is defined.

Example Object:

```json
{
    "engine": "Python",          // Optional, default "Python"
    "file": "script.py",         // Required, path to Python script
    "params": {},                // Optional, parameters passed to the script
    "callback": "main",          // Optional, default "main"
    "template": "template.ntpl"  // Optional, template to process the result
}
```

The keys "file", "params" and "template" accept variables `{:;varname:}`

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

`__NEUTRAL_SCHEMA__` is read-only for accessing the schema. Access to `__NEUTRAL_SCHEMA__` can be slow, it is faster to use parameters.

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
{:obj; {:flg; inline :} { "file": "script.py" } >>
    {:;local::varname:}
:}
```

Output:
```html
Hello from Python!
```

We can define a template in the object, we can put the template inline after ">>" or we can do both things, in the latter case they are summed, first the template defined in the object will be shown and then the inline one.

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

Flags
-----

```html
{:obj; {:flg; inline :} >> ... :}
```

### Flag: inline

Allows embedding the configuration object directly in the template instead of loading from a file:

```html
{:obj; {:flg; inline :}
    {
        "engine": "Python",
        "file": "script.py",
        "params": {
            "name": "World"
        },
        "callback": "main",
        "template": "greeting.ntpl"
    }
    >>
:}
```

Note: Using "flags" requires ">>" even when it's empty.

Examples
--------

Basic usage with file:
```html
{:obj; objfile.json :}
```

Inline configuration with parameters:
```html
{:obj; {:flg; inline :}
    {
        "file": "scripts/hello.py",
        "params": {
            "name": "World"
        }
    }
    >>
:}

{:obj; {:flg; inline :}
    {
        "file": "scripts/hello.py",
        "params": {
            "name": "{:;varname:}"
        }
    }
    >>
:}
```

Using template with script output:
```html
{:obj; {:flg; inline :}
    {
        "file": "scripts/data.py",
        "template": "templates/view.ntpl"
    }
    >>
:}
```

Limitations
------------

Python is slow and executing Python as a subprocess is even slower, use "{:cache;" when possible.

It is not the same to use "obj" to replace multiple variables than to, for example, create a complete form, in the first case performance will be affected until it is unacceptable, in the second case the loss is likely not noticeable.

---
