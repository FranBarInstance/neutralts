{:debug; ... :}
===============

Outputs the schema key if debug is enabled.

```html
{:debug; data->key :}
```

For security, debug should be disabled in production. Even so, remove "debug" bifs or wrap them to prevent execution in production:

```html
{:bool; user_rol_developper >>
    {:debug; full-schema :}
:}
```

In the previous example, your application will be responsible for managing "user_rol_developper" or any other variable used.

Stray debug bifs show an error message with a reminder to be removed in production.

Modifiers:
----------

```html
{:^debug; ... :}
{:!debug; ... :}
```
### Modifier: ^ (upline)

Eliminates previous whitespaces, (See "unprintable" for examples.)

### Modifier: ! (not):

Does not display output, except for "unprintable" if debug is active, nor does it show any error.

```html
{:!debug; this is ignored :}
```
It is useful to determine if debug is enabled:

```html
{:!debug; :}{:else; debug is disabled :}{:else; debug is enabled :}
```

The following does not display output or any error:

```html
{:!debug; :}
```

Although any key is ignored, it might be confusing to do this:

```html
{:!debug; data->key :}
```

No flags
--------

Enabling debug
------------

For debug to be active, a file must exist and its modification date must not be expired.

The file name and expiration are set in the "schema" configuration:

```text
{
    "config": {
        "debug_expire": 36000,
        "debug_file": ""
    }
}
```

If "debug_file" is empty or the file does not exist, debug is disabled.

If the file's modification date exceeds "debug_expire" in seconds, debug is disabled.

The following example enables debug for 1 hour:

```text
{
    "config": {
        "debug_expire": 3600,
        "debug_file": "/tmp/enable-neutral-debug-h5hdf7sj34xp"
    }
}
```

Then:

```text
touch /tmp/enable-neutral-debug-h5hdf7sj34xp
```

If we want to extend the debug duration, we do again:

```text
touch /tmp/enable-neutral-debug-h5hdf7sj34xp
```

Any file name can be used, but it is a good idea to include a random part.

For production, the recommended configuration is:

```text
{
    "config": {
        "debug_expire": 0,
        "debug_file": ""
    }
}
```

Examples
--------

There are global data and keys that are available to the entire template, and local data and keys that are available in each block. "Local" information must be preceded by "local::". Some keys are always "local" such as "snippets", "locale", and others.

Displays the full schema, including internal Neutral keys:

```html
{:debug; full-schema :}
```

Displays errors at this point:

```html
{:debug; __error :}
```

Displays the configuration:

```html
{:debug; config :}
```

Displays the context:

```html
{:debug; data->CONTEXT :}
```

Displays all global data:

```html
{:debug; data :}
```

Displays the variable varname:

```html
{:debug; data->varname :}
```

Displays all local data:

```html
{:debug; local::data :}
```

Displays the local variable varname:

```html
{:debug; local::data->varname :}
```

Displays the "snippets" active at this point:

```html
{:debug; local::snippets :}
```

Displays "locale":

```html
{:debug; local::locale :}
```

Displays the active translations at this point:

```html
{:debug; local::locale->trans :}
```

Displays the current language:

```html
{:debug; local::locale->current :}
```

Displays the "declare" active at this point:

```html
{:debug; local::declare :}
```

Displays the "params" active at this point:

```html
{:debug; local::params :}
```

If cache is active, the output will also be saved to the cache. If you wish to avoid this behavior, you can do so like this:

```html
{:!cache;
    {:debug; data :}
:}
```

---
