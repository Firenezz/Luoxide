use luoxide_parser::parser::compile_expression;

#[allow(unused_must_use)]
fn main() {
    tracing_subscriber::fmt::init();

    let result = compile_expression("expression");

    

    dbg!(result);
}