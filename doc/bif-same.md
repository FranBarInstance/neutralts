{:same; ... :}
==============

Output code if same.

```html
{:same; /literal/literal/ >> code :}

{:same; /{:;varname:}/42/ >> ... :}
```
Any delimiter can be used:

```html
{:same; ~a~b~ >> ... :}
{:same; #a#b# >> ... :}
{:same; |a|b| >> ... :}
{:same; XaXbX >> ... :}
...
```

Modifiers:
----------

```html
{:^same; ... :}
{:!same; ... :}
{:+same; ... :}
```

### Modifier: ^ (upline)

Eliminates previous whitespaces.

### Modifier: ! (not)

Returns the code if the two literals are NOT the same.

### Modifier: + (scope)

For more details about the "+" modifier see "modifiers".

---
