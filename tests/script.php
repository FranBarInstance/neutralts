<?php
function main($params = []) {
    $schema = $GLOBALS['__NEUTRAL_SCHEMA__'] ?? null;
    $test_nts = "";

    if (is_array($schema)
        && isset($schema["data"])
        && is_array($schema["data"])
        && isset($schema["data"]["__test-nts"])) {
        $test_nts = (string) $schema["data"]["__test-nts"];
    }

    return [
        "data" => [
            "php_hello" => "Hello from PHP!",
            "param1" => $params["param1"] ?? "",
            "test_nts" => $test_nts,
        ],
    ];
}
