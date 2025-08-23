{:fetch; ... :}
===============

Perform a JS fetch request.

```text
{:fetch; |url|event|wrapperId|class|id|name| >> code :}
```
Code is the html that will be displayed before performing the `fetch`, it can be a message, a button or the fields of a form.

* url: url to perform fetch, mandatory.
* event: can be; none, form, visible, click, auto, default auto.
* wrapperId: alternative container wrapper ID, default none
* class: add to container class, default none
* id: container ID, default none
* name: container name, default none

The url is the only mandatory parameter:

```text
{:fetch; "/url" >> <div>loading...</div> :}
```

Any delimiter can be used, but a delimiter is always required, even if only one parameter is used.

```text
{:fetch; "url" >> ... :}
{:fetch; ~url~event~ >> ... :}
{:fetch; #url#event# >> ... :}
{:fetch; |url|event| >> ... :}
{:fetch; 'url'event' >> ... :}
...
```

The reason you can use different delimiters is to use one that does not appear in the parameter, using `/` would cause problems with the `url` so we use `"` or any other:

```diff
-{:fetch; //url/ >> ... :} {:* error *:}
+{:fetch; "/url" >> ... :}
```

Modifiers:
----------

```text
{:^fetch; ... :}
```

### Modifier: ^ (upline)

Eliminates previous whitespaces, (See "unprintable" for examples.)

No flags
--------

event auto
----------

Performs the `fetch` automatically on page load:

```text
{:fetch; |/url|auto| >> <div>loading...</div> :}
```

event none
----------

No event, waiting for you to provide a custom event to perform the `fetch`:

```text
{:fetch; |/url|none| >> ... :}
```

Do not confuse `none` with leaving the parameter empty, leaving it empty would be `auto` by default.

event click
-----------

Perform the `fetch` on click:

```text
{:fetch; |/url|click| >> <button type="button">Click Me!</button> :}
```

event visible
-------------

Performs the `fetch` when the containing element is visible, it can be useful to display content in modals, images or on the scroll:

```text
{:fetch; |/url|visible| >> <div>loading...</div> :}
```

event form
----------

The `form` event generates a form:

```text
{:fetch; |/url|form| >>
    <input type="text" name="paramValue">
    <button type="submit">Send</button>
:}
```

The above will automatically generate HTML similar to this:

```text
<form class="neutral-fetch-form" method="POST" action="/url">
    <input type="text" name="paramValue">
    <button type="submit">Send</button>
</form>
```

And the form will be sent with `fetch` formatting the parameters automatically, even if file uploads are included.

neutral.js
----------

By default when you first call `fetch` the necessary JavaScript script is automatically added to the end of `body`.

This behavior can be changed in `schema` with `disable_js` to `true` for performance reasons:

```text
{
    "config": {
        "disable_js": true
    }
}
```

In this case you will have to manually add to your template at the end of `body` the script:

```text
<html>
    <body>
        ...
        <script src="neutral.min.js"></script>
    </body>
</html>
```

You can download it here: [neutral.min.js](https://gitlab.com/neutralfw/neutralts/-/tree/master/js)

HTTP Header
-----------

In each fetch request the variable `requested-with-ajax` is set to `fetch`, this allows to identify when a request comes from `ajax`.

```text
    headers:{
        'requested-with-ajax': 'fetch'
    }
```

Subsequently, your application can be assigned to a schema variable.

JS Variables
------------

The following variables are available for the forms:

```text
<script>
    var neutral_submit_loading = '...';
    var neutral_submit_timeout = 30000;
    var neutral_submit_error = '{:trans; Form ERROR! :}';
    var neutral_submit_error_delay = 3500;
    var neutral_submit_delay = 250;
</script>
```

An should be set before loading the `neutral.js` so `<head>` is a good place to do it:

* neutral_submit_loading: It is added to the end of the text of the submit button.
* neutral_submit_timeout: Timeout for form submission.
* neutral_submit_error: Error message when sending the form.
* neutral_submit_error_delay: Time it takes for the error message to disappear.
* neutral_submit_delay: Delay for form submission, prevent double click.

JS api
------

You can also use the `Neutral TS` JS api directly from JavaScript.

### Events

* neutralFetchCompleted: Sent when a fetch request has been completed successfully.
* neutralFetchError: Sent when a fetch request has produced an error.

The following parameters are available for events:

* detail.element: element
* detail.url: original url

Examples:

```javascript
<script>
    window.addEventListener('neutralFetchCompleted', function(evt) {
        console.log(evt.detail.url);
        console.log(evt.detail.element);
    });
    window.addEventListener('neutralFetchError', function(evt) {
        console.log(evt.detail.url);
        console.log(evt.detail.element);
    });
</script>
```

### Class

* neutral-fetch-form: Performs a fetch request from a form.
* neutral-fetch-auto: Performs a fetch request when the page loads.
* neutral-fetch-click: Performs a fetch request when click.
* neutral-fetch-none: Does nothing.

### Functions

* neutral_fev(): Reassign events for classes, may be necessary after fetch or xhr.
* neutral_fetch(element, url, wrapper): Performs a fetch request.
* neutral_fetch_form(form, url, wrapper): Send a form by fetch.

### Example

The following example snippet to be placed in just above `</body>`.

```javascript
{:snippet; neutral.js >>
    <script>

        // Spinner loading form
        var neutral_submit_loading = '{:snippet; spin-1x :}';

        // Timeout for send form
        var neutral_submit_timeout = 9000;

        // Translate error message
        var neutral_submit_error = '{:trans; Form ERROR! :}';

        // Time message remains in case of error
        var neutral_submit_error_delay = 5500;

        // Prevent double clicking
        var neutral_submit_delay = 300;

        // Execute the scripts contained in the fetch avoiding arbitrary code.
        // - Executes only it is not an external URL.
        // - Executes only those in a container with the script-container class.
        window.addEventListener('neutralFetchCompleted', function(evt) {
            if (!evt.detail.url.includes('://')) {
                const scriptContainer = evt.detail.element.querySelector('.script-container');
                if (scriptContainer) {
                    const scripts = scriptContainer.querySelectorAll('script');
                    scripts.forEach(script => {
                        const newScript = document.createElement('script');
                        newScript.text = script.textContent;
                        document.body.appendChild(newScript);
                    });
                }
            }
        });
    </script>
    <script src="/path/to/neutral.min.js"></script>
:}
```

Then:

```html
<html>
    <head>
        ...
    </head>
    <body>
        ...
        {:snippet; neutral.js :}
    </body>
</html>
```

---
