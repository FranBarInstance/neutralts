<?php

    /*
        Neutral IPC example without framework
    */

    include 'NeutralIpcTemplate.php';
    include 'utility.php';

    // A "schema" is needed for the configuration and for the data to be presented.
    $schema_json = file_get_contents("data/schema.json");
    $schema = json_decode($schema_json, true);

    // The user's language is set
    $accepted = $schema["data"]["site"]["languages"];
    $schema["inherit"]["locale"]["current"] = negotiate_language($accepted);

    // The theme is set. Any value coming from the context (env, cookies, ...)
    // should be considered unsafe, here we will ignore it as an example of
    // how Neutral can handle this.
    $schema["data"]["site"]["theme"] = $_GET['theme'] ?? $_COOKIE['theme'] ?? "sketchy";

    // Set POST data
    $schema["data"]["POST"] = $_POST;

    // Set a framework, just as an example
    $schema["data"]["current-fw"] = "build-form";

    // Create the template
    $template_path = realpath("www/tpl/home.ntpl");
    $template = new NeutralIpcTemplate($template_path, $schema);

    // Rendered content
    $contents = $template->render();

    // If “exit” or “redirect” is used, the status codes must be managed.
    $status_code = $template->get_status_code();
    $status_text = $template->get_status_text();

    // Only in certain cases, e.g., redirect.
    $status_param = $template->get_status_param();

    $protocol = $_SERVER['SERVER_PROTOCOL'] ?? "HTTP/1.0";

   // If not changed (with "{:exit;:}" for example) the template always
   // returns a status code 200 OK.
   if ($status_code >= "400") {
       $error = [
           "data" => [
               "error" => [
                   "code" => $status_code,
                   "text" => $status_text
               ]
            ]
        ];

        // The custom error page is used.
        $template_path = realpath("www/tpl/error.ntpl");
        $template->set_path($template_path);

        // The error variables are added to the schema, just because this
        // is how we set up our custom error page.
        $template->merge_schema($error);

        // Rendered content for error custom page.
        // Be careful not to re-render the content that causes the error,
        // for example if the error occurs in a snippet that shares the error page.
        $contents = $template->render();

        // e.g. "HTTP/1.0 404 Not Found"
        header("$protocol $status_code $status_text");
        echo $contents;
    } else {
        // e.g.  "HTTP/1.0 200 OK";
        header("$protocol $status_code $status_text");
        echo $contents;
    }
