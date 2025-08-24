{:contains; ... :}
==================

Outputs code only if the haystack contains the needle.

Usage:
------

```html
{:contains; /haystack/needle/ >> code :}
{:contains; /literal/literal/ >> code :}
{:contains; /{:;varname:}/42/ >> ... :}
```

Any delimiter can be used:

```html
{:contains; ~haystack~needle~ >> ... :}
{:contains; #haystack#needle# >> ... :}
{:contains; |haystack|needle| >> ... :}
{:contains; XhaystackXneedleX >> ... :}
```

If the haystack contains the needle, the code block is rendered. Otherwise, nothing is output.

Modifiers:
----------

```html
{:^contains; ... :}
{:!contains; ... :}
```

### Modifier: ^ (upline)
Removes preceding whitespace before output. See "unprintable" for more details.

### Modifier: ! (not)
Negates the condition. Outputs code only if the haystack does NOT contain the needle.

```html
{:!contains; /haystack/needle/ >> shown if not contains :}
```

No flags
--------

Examples
--------

Basic usage:
```html
{:contains; /hello/world/ >> This will not show :}
{:contains; /hello/lo/ >> This will show :}
```

With variables:
```html
{:contains; /{:;varname:}/needle/ >> Output if varname contains 'needle' :}
```

With negation:
```html
{:!contains; /abc/xyz/ >> Shown because 'abc' does not contain 'xyz' :}
```

---
