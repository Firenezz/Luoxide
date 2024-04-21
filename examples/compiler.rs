fn main() {
    let source = r##"
6 * 1 + 2 * 5
"##;
    dbg!(luoxidant::internal::syntax::parser::parse_expression(source).unwrap());
}
