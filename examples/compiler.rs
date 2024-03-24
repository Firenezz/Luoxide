fn main() {
    let source = r##"
v = 1
"##;
    luoxidant::internal::syntax::parser::parse_chunk(source);
}
