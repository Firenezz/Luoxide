fn main() {
    let source = r##"
1 + 2
"##;
    luoxidant::internal::syntax::parser::parse_expression(source).unwrap();
}
