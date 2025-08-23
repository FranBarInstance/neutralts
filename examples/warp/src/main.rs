use warp::Filter;
use warp::http::StatusCode;
use std::convert::Infallible;
use serde_json::{json, Value};
use std::fs;
use neutralts::Template;

#[tokio::main]
async fn main() {
    let route = warp::path::end()
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(warp::cookie::optional("theme"))
        .and(warp::cookie::optional("lang"))
        .and_then(home);
    let fallback = warp::any()
        .and(warp::cookie::optional("theme"))
        .and(warp::cookie::optional("lang"))
        .and_then(handle_404);
    let routes = route.or(fallback);

    warp::serve(routes).run(([127, 0, 0, 1], 9090)).await;
}

async fn home(
    params: std::collections::HashMap<String, String>,
    theme_cookie: Option<String>,
    lang_cookie: Option<String>,
) -> Result<impl warp::Reply, Infallible> {
    let theme_param = params.get("theme").cloned().unwrap_or_else(|| "".to_string());
    let lang_param = params.get("lang").cloned().unwrap_or_else(|| "".to_string());
    let theme_cookie = theme_cookie.unwrap_or_else(|| "".to_string());
    let lang_cookie = lang_cookie.unwrap_or_else(|| "".to_string());


    // A "schema" is needed for the configuration and for the data to be presented.
    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();

    // The user's language is set
    if !lang_param.is_empty() {
        schema["inherit"]["locale"]["current"] = lang_param.into();
    } else if !lang_cookie.is_empty() {
        schema["inherit"]["locale"]["current"] = lang_cookie.into();
    } else {
        schema["inherit"]["locale"]["current"] = "en".into();
    }

    // The theme is set. Any value coming from the context (env, cookies, ...)
    // should be considered unsafe, here we will ignore it as an example of
    // how Neutral can handle this.
    if !theme_param.is_empty() {
        schema["data"]["site"]["theme"] = theme_param.into();
    } else if !theme_cookie.is_empty() {
        schema["data"]["site"]["theme"] = theme_cookie.into();
    } else {
        schema["data"]["site"]["theme"] = "sketchy".into();
    }

    // Set a framework, just as an example
    schema["data"]["current-fw"] = json!("warp");

    // Create the template
    let template_path = "../../examples/www/tpl/home.ntpl";
    let mut template = Template::from_file_value(&template_path, schema.clone()).unwrap();

    // Rendered content
    let contents = template.render();

    // If “exit” or “redirect” is used, the status codes must be managed.
    let status_code = template.get_status_code().clone();
    let status_text = template.get_status_text().clone();

    // Only in certain cases, e.g., redirect.
    let _status_param = template.get_status_param().clone();

    // Convertir el string a u16
    let status_u16: u16 = status_code.parse().unwrap();  // Puede fallar si el string no es válido

    // Crear el estado con el código numérico
    let status = StatusCode::from_u16(status_u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    // If not changed (with "{:exit;:}" for example) the template always
    // returns a status code 200 OK.
    if status_code.as_str() >= "400" {
        let error = json!({
            "data": {
                "error": {
                    "code": status_code,
                    "text": status_text
                }
            }
        });

        // The custom error page is used.
        template.set_src_path("../../examples/www/tpl/error.ntpl").unwrap();

        // The error variables are added to the schema, just because this
        // is how we set up our custom error page.
        template.merge_schema_value(error);

        // Rendered content for error custom page.
        // Be careful not to re-render the content that causes the error,
        // for example if the error occurs in a snippet that shares the error page.
        let error_contents = template.render();

        // The response is sent with the corresponding error.
        Ok(warp::reply::with_status(warp::reply::html(error_contents), status))
    } else {
        Ok(warp::reply::with_status(warp::reply::html(contents), StatusCode::OK))
    }

}

// Función para manejar el error 404.
async fn handle_404(
    theme_cookie: Option<String>,
    lang_cookie: Option<String>,
) -> Result<impl warp::Reply, Infallible> {
    let theme_cookie = theme_cookie.unwrap_or_else(|| "".to_string());
    let lang_cookie = lang_cookie.unwrap_or_else(|| "".to_string());

    let schema_str = &fs::read_to_string("../../examples/data/schema.json").expect("schema is required");
    let mut schema: Value = serde_json::from_str(schema_str).unwrap();
    schema["inherit"]["locale"]["current"] = "en".into();
    if !lang_cookie.is_empty() {
        schema["inherit"]["locale"]["current"] = lang_cookie.into();
    } else {
        schema["inherit"]["locale"]["current"] = "en".into();
    }
    if !theme_cookie.is_empty() {
        schema["data"]["site"]["theme"] = theme_cookie.into();
    } else {
        schema["data"]["site"]["theme"] = "sketchy".into();
    }

    // The error variables are added to the schema, just because this
    // is how we set up our custom error page.
    schema["data"]["error"] = json!({
        "code": "404",
        "text": "Not Found"
    });

    let template_path = "../../examples/www/tpl/error.ntpl";
    let mut template = Template::from_file_value(&template_path, schema).unwrap();
    let contents = template.render();

    Ok(warp::reply::with_status(warp::reply::html(contents), StatusCode::NOT_FOUND))
}
