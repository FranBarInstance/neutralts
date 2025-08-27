#![doc = include_str!("../../doc/bif-moveto.md")]

use crate::{bif::constants::*, bif::Bif, bif::BifError, constants::*, json};
use md5::{Digest, Md5};

impl<'a> Bif<'a> {
    /*
        {:moveto; <tag >> ... :}
        {:moveto; </tag >> ... :}
    */
    pub(crate) fn parse_bif_moveto(&mut self) -> Result<(), BifError> {
        if self.mod_filter || self.mod_negate || self.mod_scope {
            return Err(self.bif_error(BIF_ERROR_MODIFIER_NOT_ALLOWED));
        }

        if self.inherit.in_cache {
            self.out = format!("{}{}{}", "{:!cache;", self.raw.to_string(), ":}");
        } else {
            self.out = EMPTY_STRING;
        }

        self.extract_params_code(true);

        if !self.flags.is_empty() {
            return Err(self.bif_error(BIF_ERROR_FLAGS_NOT_ALLOWED));
        }

        if self.code.contains(BIF_OPEN) {
            self.code = new_child_parse!(self, &self.code, self.mod_scope);
        }

        self.moveto(&self.params.clone(), &self.code.clone());

        Ok(())
    }

    pub(crate) fn moveto(&mut self, to: &str, code: &str) {
        let mut moveto = json!({});
        let mut hasher = Md5::new();

        // the same code moves only once
        hasher.update(code.replace("\n", "").replace(" ", ""));
        let code_hash = hasher.finalize();
        let code_hash = format!("{:x}", code_hash);

        moveto[to] = json!(code);
        self.shared.schema["__moveto"][&code_hash] = moveto;
    }
}

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
