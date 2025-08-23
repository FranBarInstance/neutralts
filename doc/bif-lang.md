{:lang; ... :}
================

Returns the current language defined in the schema. The value is taken from `inherit.locale.current`.

Example:

**Schema**
```json
{
    "inherit": {
        "locale": {
            "current": "en"
        }
    }
}
```

**Template**
```html
<html lang="{:lang;:}">
...
</html>
```

**Result**
```html
<html lang="en">
...
</html>
```

No modifiers
----------


No flags
--------
