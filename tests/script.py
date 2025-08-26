"""Test neutral python objects."""

def main(params=None):
    """Test neutral python objects."""

    if params is None:
        params = {}

    return {
        "data": {
            "py_hello": "Hello from Python!",
            "param1": params.get("param1", ""),
        }
    }
