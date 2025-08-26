{:obj; ... :}
=============

Description...

```html
{:obj; objfile.json :}
```

Obj

```html
{
    "engine": "Python",           // optional default "Python"
    "file": "script.py",          // required
    "params": {},                 // optional
    "callback": "main",           // optional default "main"
    "template": "template.ntpl"   // optional
}
```

Script

```Python
def main(params=None):
    schema = globals().get('__NEUTRAL_SCHEMA__')

    return {
        "data": {
            "get-var": schema["data"]["CONTEXT"]["GET"]["var"]
        }
    }
```

Modifiers:
----------

```html
{:^obj; ... :}
{:+obj; ... :}
...
```
### Modifier: ^ (upline)

...

### Modifier: + (scope)

...

Flags
-----

```html
{:obj; {:flg; inline :} >> ... :}
```

### Flag: inline

```html
{:obj; {:flg; inline :}
    {
        "engine": "Python",
        "file": "script.py",
        "params": {},
        "callback": "main",
        "template": "template.ntpl"
    }
    >>
    ...
:}
```

Examples
--------



---
