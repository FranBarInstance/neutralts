#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    #[test]
    fn test_bif_moveto_head() {
        let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; <head >> <script></script> :}
    "#.trim();
        let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head><script></script>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, out);
    }

    #[test]
    fn test_bif_moveto_head_once() {
        let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; <head >> <script></script> :}
        {:moveto; <head >> <script></script> :}
        {:moveto; <head >> <script></script> :}
    "#.trim();
        let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head><script></script>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, out);
    }

    #[test]
    fn test_bif_moveto_head_ends() {
        let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; </head >> <script></script> :}
    "#.trim();
        let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            <script></script></head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, out);
    }

    #[test]
    fn test_bif_moveto_body_ends() {
        let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; </body >> <script></script> :}
    "#.trim();
        let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            <script></script></body>
        </html>
    "#.trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, out);
    }

    #[test]
    fn test_bif_moveto_body() {
        let source = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
        {:moveto; <body >> <script></script> :}
    "#.trim();
        let out = r#"
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="UTF-8">
                <title>Lorem Ipsum</title>
                <style></style>
            </head>
            <body><script></script>
                <div class="container">
                    <h1>Lorem ipsum</h1>
                    <p>
                        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam vehicula turpis nec
                        libero volutpat, at tincidunt dui cursus. Maecenas euismod, diam nec fringilla malesuada,
                        justo tellus gravida sapien, nec sollicitudin nulla odio a lorem.
                    </p>
                </div>
            </body>
        </html>
    "#.trim();
        let mut template = match crate::Template::new() {
            Ok(tpl) => tpl,
            Err(error) => {
                println!("Error creating Template: {}", error);
                assert!(false);
                return;
            }
        };
        template.merge_schema_str(SCHEMA).unwrap();
        template.set_src_str(source);
        let result = template.render();
        assert!(!template.has_error());
        assert_eq!(result, out);
    }
}
