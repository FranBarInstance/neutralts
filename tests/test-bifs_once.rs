use neutralts::test_helpers::*;

#[test]
fn test_bif_comments() {
    let schema = r#"
{
    "config": {
        "comments": "keep"
    }
}
"#
    .trim();

    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("<div>{:* comment *:}</div>");
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments_multiline() {
    let schema = r#"
{
    "config": {
        "comments": "keep"
    }
}
"#
    .trim();
    let source = r#"
    {:*

        test comment

    *:}
    <div></div>
"#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments_nested() {
    let schema = r#"
{
    "config": {
        "comments": "keep"
    }
}
"#
    .trim();
    let source = r#"
    {:*
        test comment
        {:* comment *:}
    *:}
    <div></div>
"#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_comments_complex() {
    let schema = r#"
{
    "config": {
        "comments": "keep"
    }
}
"#
    .trim();
    let source = r#"
    {:* comment *:}
    {:* {:code; *:}
        {:code;
            {:* comment *:}
            <div>{:; {:* comment *:} __test-nts {:* comment *:} :}</div>
        :}
    {:* :} *:}
"#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_comments_remove() {
    let schema = r#"
{
    "config": {
        "comments": "remove"
    }
}
"#
    .trim();
    let source = r#"
    {:* comment *:}
    {:* {:code; *:}
        {:code;
            {:* comment *:}
            <div>{:; {:* comment *:} __test-nts {:* comment *:} :}</div>
        :}
    {:* :} *:}
"#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str(source);
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div>nts</div>");
}

#[test]
fn test_bif_unprintable() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:;:}</div>");
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_unprintable_spaces() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div> {:;:} </div>");
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div>  </div>");
}

#[test]
fn test_bif_unprintable_upline() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("|  \n  {:^;:}<div></div>");
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "|<div></div>");
}

#[test]
fn test_bif_unprintable_comments() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("{:; {:* comment *:} :}<div></div>");
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_get_errors() {
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.set_src_str("<div>{:for; n :}{:allow: none :}</div>");
    let result = template.render_once();
    let errors = template.get_error();
    assert!(template.has_error());
    assert!(errors[0].to_string().contains("arguments not found"));
    assert!(errors[1].to_string().contains("delimiter was not found"));
    assert_eq!(result, "<div></div>");
}

#[test]
fn test_bif_complete_tpl() {
    let schema = r#"
    {
        "config": {
            "comments": "keep"
        },
        "inherit": {
            "locale": {
                "current": "es",
                "trans": {
                    "es": {
                        "Title": "Título",
                        "ref:greeting-nts": "¡Hola!"
                    }
                }
            }
        }
    }
    "#
    .trim();
    let out = r#"
<!DOCTYPE html>
<html lang=es>
<head>
    <meta charset=UTF-8>
    <title>Lorem Ipsum Ok nts</title>
</head>
    <body>
        <div class="container">
            <h1>Lorem ipsum Título</h1>
            <div>
                nts
            </div>
                1:nts none
                2:
                    3:hello data nts
                        4:hello data nts
                            5:hello data nts
                                6:hello data nts
                                6
                            5
                        4
                    3
                2
                1:nts none
            <div>
        Lorem:
                Ipsum:
                        Dolor:
                                Sit:
                                        Amet=Consectetur adipiscing elit.
                                        Sed=Do eiusmod tempor incididunt.
                                        Ut=Labore et dolore magna aliqua.
                                        Array:
                                                0=Lorem
                                                1=Ipsum
                                                2=Dolor
                                Enim:
                                        Ad=Minim veniam, quis nostrud exercitation.
                                        Ullamco=Laboris nisi ut aliquip ex ea commodo consequat.
                                        Array:
                                                0=Sed
                                                1=Do
                                                2=Eiusmod
                                                3=Tempor
                                                4=Incididunt
                        Irure:
                                Dolor:
                                        In=Reprehenderit in voluptate.
                                        Excepteur=Sint occaecat cupidatat.
                                        Array:
                                                0=Ut
                                                1=Enim
                                                2=Ad
                Officia:
                        Deserunt:
                                Mollit:
                                        Anim:
                                                Id=Est laborum et dolorum fugiat nulla pariatur.
                                                Sed=Quis nostrud exercitation.
                                Commodo:
                                        Consequat=Duis aute irure dolor in reprehenderit.
                                        Array:
                                                0=Amet
                                                1=Sed
                                                2=Do
                                                3=Eiusmod
                                                4=Tempor
            </div>
            <div>
                is not traversal
            </div>
            <div>
                    num 0 = 0
                    num 1 = 1
                    num 2 = 2
                    num 3 = 3
                    num 4 = 4
                    num 5 = 5
                    num 6 = 6
                    num 7 = 7
                    num 8 = 8
                    num 9 = 9
            </div>
        </div>
    </body>
</html>
"#
    .trim();
    let mut template = match neutralts::Template::new() {
        Ok(tpl) => tpl,
        Err(error) => {
            println!("Error creating Template: {}", error);
            assert!(false);
            return;
        }
    };
    template.merge_schema_str(SCHEMA).unwrap();
    template.merge_schema_str(schema).unwrap();
    template.set_src_str("{:include; tests/complete.ntpl :}");
    let result = template.render_once();
    assert!(!template.has_error());
    assert_eq!(template.get_status_code(), "200");
    assert_eq!(template.get_status_text(), "OK");
    assert_eq!(template.get_status_param(), "");
    assert_eq!(result, out);
}
