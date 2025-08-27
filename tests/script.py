"""Test neutral python objects."""

def main(params=None):
    """Test neutral python objects."""

    if params is None:
        params = {}

    try:
        test_nts = globals()['__NEUTRAL_SCHEMA__']['data']['__test-nts']
    except (KeyError, TypeError):
        test_nts = ""

    return {
        "data": {
            "py_hello": "Hello from Python!",
            "param1": params.get("param1", ""),
            "test_nts": test_nts
        }
    }
