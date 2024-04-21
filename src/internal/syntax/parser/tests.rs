use super::*;

#[test]
fn parse_file() {
    let _input = r##"
-5
"##;

    //println!("{}", tokens);
}

macro_rules! check_expr {
    ($input:literal) => {
        let input = $input;
        let interner = Rc::from(DefaultInterner::default());
        let lexer = Lexer::new(input, interner.clone());
        match Parser::new(lexer, interner).parse_expression() {
            Ok(module) => assert_debug_snapshot!(module),
            Err(_) => {
                //eprintln!("{}", err.report(input, true));
                panic!("Failed to parse source, see errors above.")
            }
        };
    };
}

#[test]
fn unary_expr() {
    check_expr!(r#"-a"#);
    check_expr!(r#"not a"#);
    check_expr!(r#"#a"#);
    check_expr!(r#"~a"#);
}
