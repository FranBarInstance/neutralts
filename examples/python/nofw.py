import sys
import os
import re
import json

#
# Neutral IPC example without framework
#

# NeutralIpcTemplate directory location
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), './')))
from NeutralIpcTemplate import NeutralIpcTemplate

# A "schema" is needed for the configuration and for the data to be presented.
with open("../../examples/data/schema.json", "r", encoding="utf-8") as file:
    schema_json = file.read()
schema = json.loads(schema_json)

# The user's language is seto
schema["inherit"]["locale"]["current"] = "en"

# The theme is set.
schema["data"]["site"]["theme"] = "sketchy"

# Set a framework, just as an example
schema["data"]["current-fw"] = "python"

# Create the template
template_path = os.path.realpath("../../examples/www/tpl/home.ntpl")
template = NeutralIpcTemplate(template_path, schema)

# Rendered content
contents = template.render()

# If “exit” or “redirect” is used, the status codes must be managed.
status_code = template.get_status_code()
status_text = template.get_status_text()
status_param = template.get_status_param()

protocol = os.getenv('SERVER_PROTOCOL', 'HTTP/1.0')

# If not changed (with "{:exit;:}" for example) the template always
# returns a status code 200 OK.
if int(status_code) >= 400:
    error = {
        "data": {
            "error": {
                "code": status_code,
                "text": status_text
            }
        }
    }

    # The custom error page is used.
    template.set_path("../../examples/www/tpl/error.ntpl")

    # The error variables are added to the schema, just because this
    # is how we set up our custom error page.
    template.merge_schema(error)

    # Rendered content for error custom page.
    # Be careful not to re-render the content that causes the error,
    # for example if the error occurs in a snippet that shares the error page.
    contents = template.render()

    # e.g.: "HTTP/1.0 404 Not Found"
    print(f"{protocol} {status_code} {status_text}")
    print(contents)
else:
    # e.g.: "HTTP/1.0 200 OK"
    print(f"{protocol} {status_code} {status_text}")
    print(contents)
