Plugin example
--------------

Countries form field provides a field with the list of countries for one form.

As an example it has been implemented in two ways, to call it as an include and as a snippet, in practice, it is sufficient to do it only one way.

Usage
-----

We can include field.ntpl with default values:

```
<form>
    <label for="country">Country:</label>
        {:include; path/to/countries-form-field/field.ntpl :}
    <input type="submit" value="Submit">
</form>
```

Output:

```
<form>
    <label for="country">Country:</label>
    <select class="countries-form-field">
        <option value="">Select a country</option>
        <option  value="AF">Afghanistan</option>
        <option  value="AX">Åland Islands</option>
        <option  value="AL">Albania</option>
        ...
    </select>
    <input type="submit" value="Submit">
</form>

```

We can include field.ntpl with parameters:

```
<form>
    <label for="country">Country:</label>
        {:code;
            {:param; select-prop >> class="countries" :}
            {:param; option-prop >> class="countries-opt" :}

            {:include; path/to/countries-form-field/field.ntpl :}
        :}
    <input type="submit" value="Submit">
</form>
```

We can use snippet with default values:

```
{:include; path/to/countries-form-field/snippets.ntpl :}

<form>
    <label for="country">Country:</label>
        {:snippet; countries-form-field :}
    <input type="submit" value="Submit">
</form>
```

We can use snippet with parameters:

```
{:include; path/to/countries-form-field/snippets.ntpl :}

<form>
    <label for="country">Country:</label>
        {:code;
            {:param; select-prop >> class="countries" :}
            {:param; option-prop >> class="countries-opt" :}

            {:snippet; countries-form-field :}
        :}
    <input type="submit" value="Submit">
</form>
```

If you are trying to implement a small utility like this, you can do it as an include, but usually a set of utilities or library is usually implemented and in this case it is better to host them in a snippets file, include it, and then call each snippet as needed.

You can see the code of the example to know how it is done.
